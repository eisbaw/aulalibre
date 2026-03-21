//! FUSE filesystem trait implementation.
//!
//! Implements `fuser::Filesystem` to expose Aula data as a read-only
//! directory tree. Uses `Arc<Mutex<Session>>` for thread safety and
//! `tokio::runtime::Handle::block_on` for async API calls from sync
//! FUSE callbacks.
//!
//! # Safety invariant: Mutex held across `block_on`
//!
//! Throughout this file, `session.lock_or_recover()` is held while calling
//! `Handle::block_on()`. This is safe because `fuser` dispatches filesystem
//! callbacks on its own dedicated (non-async) threads — `block_on` will never
//! be called from within a tokio async context, so it cannot deadlock the
//! runtime. The `Mutex` serializes API calls, which is acceptable for a
//! single-user mount. **Do not call these methods from an async task** — that
//! would panic (`block_on` inside a runtime) or deadlock.

use std::ffi::OsStr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fuser::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use log::{debug, error, warn};
use tokio::runtime::Handle;

use aula_api::models::calendar::EventSimpleDto;
use aula_api::models::calendar::GetEventsParameters;
use aula_api::models::documents::{GetSecureDocumentsArguments, SecureDocumentDto};
use aula_api::models::gallery::{AlbumDto, GalleryViewFilter};
use aula_api::models::messaging::{GetThreadListArguments, MessageThreadSubscription};
use aula_api::models::notifications::NotificationItemDto;
use aula_api::models::posts::{GetPostApiParameters, PostApiDto};
use aula_api::models::presence::ChildStatusDto;
use aula_api::Session;

use crate::cache::{Cache, CacheKey, LIST_TTL, PRESENCE_TTL};
use crate::inode_table::{ContentSource, InodeEntry, InodeTable, ResourceType};
use crate::sanitize::dir_name;
use crate::timestamp::{mtime_from, parse_aula_datetime};

/// Extension trait for `Mutex` that recovers from poisoned locks instead of
/// panicking. In a FUSE filesystem, a panic crashes the mount — returning
/// potentially-stale data is strictly preferable.
trait MutexExt<T> {
    fn lock_or_recover(&self) -> MutexGuard<'_, T>;
}

impl<T> MutexExt<T> for Mutex<T> {
    fn lock_or_recover(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(|poisoned| {
            error!("Mutex poisoned, recovering inner data");
            poisoned.into_inner()
        })
    }
}

/// TTL for FUSE attribute/entry caching (how long the kernel caches metadata).
const FUSE_TTL: Duration = Duration::from_secs(60);

/// Default directory permissions (read + execute for owner).
const DIR_PERM: u16 = 0o555;

/// Default file permissions (read for owner).
const FILE_PERM: u16 = 0o444;

/// Items per page for paginated API calls.
const PAGE_SIZE: i32 = 20;

/// The Aula FUSE filesystem.
pub struct AulaFs {
    session: Arc<Mutex<Session>>,
    rt: Handle,
    inodes: Mutex<InodeTable>,
    cache: Mutex<Cache>,
    uid: u32,
    gid: u32,
}

impl AulaFs {
    pub fn new(session: Arc<Mutex<Session>>, rt: Handle) -> Self {
        let uid = unsafe { libc::getuid() };
        let gid = unsafe { libc::getgid() };
        Self {
            session,
            rt,
            inodes: Mutex::new(InodeTable::new()),
            cache: Mutex::new(Cache::new()),
            uid,
            gid,
        }
    }

    /// Build a FileAttr for a directory.
    fn dir_attr(&self, ino: u64, mtime: SystemTime) -> FileAttr {
        FileAttr {
            ino,
            size: 0,
            blocks: 0,
            atime: mtime,
            mtime,
            ctime: mtime,
            crtime: mtime,
            kind: FileType::Directory,
            perm: DIR_PERM,
            nlink: 2,
            uid: self.uid,
            gid: self.gid,
            rdev: 0,
            blksize: 512,
            flags: 0,
        }
    }

    /// Build a FileAttr for a regular file.
    fn file_attr(&self, ino: u64, size: u64, mtime: SystemTime) -> FileAttr {
        FileAttr {
            ino,
            size,
            blocks: size.div_ceil(512),
            atime: mtime,
            mtime,
            ctime: mtime,
            crtime: mtime,
            kind: FileType::RegularFile,
            perm: FILE_PERM,
            nlink: 1,
            uid: self.uid,
            gid: self.gid,
            rdev: 0,
            blksize: 512,
            flags: 0,
        }
    }

    /// Get a FileAttr for an inode entry.
    fn attr_for(&self, ino: u64, entry: &InodeEntry) -> FileAttr {
        match entry {
            InodeEntry::Root => self.dir_attr(ino, SystemTime::now()),
            InodeEntry::ResourceDir(_) => self.dir_attr(ino, SystemTime::now()),
            InodeEntry::ResourceItem { modified, .. } => self.dir_attr(ino, *modified),
            InodeEntry::File { size, mtime, .. } => self.file_attr(ino, *size, *mtime),
            InodeEntry::PageDir { .. } => self.dir_attr(ino, SystemTime::now()),
        }
    }

    // =========================================================================
    // Resource population (API calls)
    // =========================================================================

    /// Populate a resource directory with data from the API.
    fn populate_resource_dir(&self, ino: u64, rt: ResourceType, page: i32) {
        match rt {
            ResourceType::Posts => self.populate_posts(ino, page),
            ResourceType::Messages => self.populate_messages(ino, page),
            ResourceType::Calendar => self.populate_calendar(ino, page),
            ResourceType::Notifications => self.populate_notifications(ino),
            ResourceType::Gallery => self.populate_gallery(ino, page),
            ResourceType::Documents => self.populate_documents(ino, page),
            ResourceType::Presence => self.populate_presence(ino),
        }
    }

    fn populate_posts(&self, parent_ino: u64, page: i32) {
        let cache_key = CacheKey::ResourceList {
            resource: "posts".into(),
            page,
        };

        // Check cache first.
        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return; // Already populated and cache is fresh.
            }
        }

        let inst_profile_ids = {
            let session = self.session.lock_or_recover();
            session.all_institution_profile_ids()
        };

        let params = GetPostApiParameters {
            parent: None,
            group_id: None,
            is_important: None,
            creator_portal_role: None,
            institution_profile_ids: Some(inst_profile_ids),
            related_institutions: None,
            own_post: false,
            is_unread: false,
            is_bookmarked: false,
            limit: Some(PAGE_SIZE),
            index: Some(page * PAGE_SIZE),
        };

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt
                .block_on(aula_api::services::posts::get_posts(&mut session, &params))
        };

        match result {
            Ok(post_result) => {
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                if let Some(posts) = &post_result.posts {
                    for post in posts {
                        self.insert_post(&mut inodes, parent_ino, post);
                    }
                }
                // Add pagination directory if there are more posts.
                if post_result.has_more_posts {
                    let next_page = page + 1;
                    let page_name = format!("page-{}", next_page + 1);
                    inodes.insert(
                        parent_ino,
                        page_name,
                        InodeEntry::PageDir {
                            resource_type: ResourceType::Posts,
                            page: next_page,
                            parent_inode: parent_ino,
                        },
                    );
                }

                // Cache the result.
                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&post_result).unwrap_or_else(|e| {
                        warn!("Failed to serialize posts to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    LIST_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch posts: {}", e);
            }
        }
    }

    fn insert_post(&self, inodes: &mut InodeTable, parent_ino: u64, post: &PostApiDto) {
        let id = post.id.unwrap_or(0);
        let title = post.title.as_deref().unwrap_or("untitled");
        let name = dir_name(id, title);

        let ctime = parse_aula_datetime(post.time_stamp.as_deref().unwrap_or(""));
        let mtime = mtime_from(post.edited_at.as_deref(), post.time_stamp.as_deref());

        let item_ino = inodes.insert(
            parent_ino,
            name,
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Posts,
                name: dir_name(id, title),
                created: ctime,
                modified: mtime,
            },
        );

        // Add content.txt (HTML body as text).
        let html = post
            .content
            .as_ref()
            .and_then(|c| c.html.as_deref())
            .unwrap_or("");
        let text = strip_html(html);
        let text_bytes = text.len() as u64;
        inodes.insert(
            item_ino,
            "content.txt".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "content.txt".to_string(),
                content: ContentSource::Text(text),
                size: text_bytes,
                mtime,
            },
        );

        // Add metadata.json.
        let json = serde_json::to_string_pretty(post).unwrap_or_else(|e| {
            warn!("Failed to serialize post metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;
        inodes.insert(
            item_ino,
            "metadata.json".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "metadata.json".to_string(),
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );
    }

    fn populate_messages(&self, parent_ino: u64, page: i32) {
        let cache_key = CacheKey::ResourceList {
            resource: "messages".into(),
            page,
        };

        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return;
            }
        }

        let args = GetThreadListArguments {
            folder_id: None,
            filter_type: None,
            sort_type: None,
            sort_order: None,
            page: Some(page),
            thread_ids: None,
            mail_box_owner_type: None,
            mail_box_owners: None,
            active_children: None,
        };

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt
                .block_on(aula_api::services::messaging::get_thread_list(
                    &mut session,
                    &args,
                ))
        };

        match result {
            Ok(thread_list) => {
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                if let Some(threads) = &thread_list.threads {
                    for thread in threads {
                        self.insert_thread(&mut inodes, parent_ino, thread);
                    }
                }
                if thread_list.more_messages_exist {
                    let next_page = page + 1;
                    let page_name = format!("page-{}", next_page + 1);
                    inodes.insert(
                        parent_ino,
                        page_name,
                        InodeEntry::PageDir {
                            resource_type: ResourceType::Messages,
                            page: next_page,
                            parent_inode: parent_ino,
                        },
                    );
                }

                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&thread_list).unwrap_or_else(|e| {
                        warn!("Failed to serialize threads to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    LIST_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
            }
        }
    }

    fn insert_thread(
        &self,
        inodes: &mut InodeTable,
        parent_ino: u64,
        thread: &MessageThreadSubscription,
    ) {
        let id = thread.id.unwrap_or(0);
        let subject = thread.subject.as_deref().unwrap_or("(no subject)");
        let name = dir_name(id, subject);

        let ctime = thread
            .latest_message
            .as_ref()
            .and_then(|m| m.send_date_time.as_deref())
            .map(parse_aula_datetime)
            .unwrap_or(UNIX_EPOCH);
        let mtime = ctime;

        let item_ino = inodes.insert(
            parent_ino,
            name.clone(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Messages,
                name,
                created: ctime,
                modified: mtime,
            },
        );

        // Add metadata.json for the thread subscription.
        let json = serde_json::to_string_pretty(thread).unwrap_or_else(|e| {
            warn!("Failed to serialize thread metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;
        inodes.insert(
            item_ino,
            "metadata.json".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "metadata.json".to_string(),
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );

        // Latest message preview as content.txt.
        let preview = thread
            .latest_message
            .as_ref()
            .and_then(|m| m.text.as_ref())
            .and_then(|t| t.html.as_deref())
            .map(strip_html)
            .unwrap_or_default();
        if !preview.is_empty() {
            let sender = thread
                .latest_message
                .as_ref()
                .and_then(|m| m.sender.as_ref())
                .and_then(|s| s.display_name.as_deref())
                .unwrap_or("Unknown");
            let text = format!("From: {}\n\n{}", sender, preview);
            let text_bytes = text.len() as u64;
            inodes.insert(
                item_ino,
                "latest.txt".to_string(),
                InodeEntry::File {
                    parent_inode: item_ino,
                    name: "latest.txt".to_string(),
                    content: ContentSource::Text(text),
                    size: text_bytes,
                    mtime,
                },
            );
        }
    }

    fn populate_calendar(&self, parent_ino: u64, _page: i32) {
        let cache_key = CacheKey::ResourceList {
            resource: "calendar".into(),
            page: 0,
        };

        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return;
            }
        }

        let inst_profile_ids = {
            let session = self.session.lock_or_recover();
            session.all_institution_profile_ids()
        };

        // Fetch events for the next 30 days.
        let now = chrono::Local::now();
        let start = now.format("%Y-%m-%d").to_string();
        let end = (now + chrono::Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();

        let params = GetEventsParameters {
            inst_profile_ids: Some(inst_profile_ids),
            resource_ids: None,
            start: Some(start),
            end: Some(end),
            specific_types: None,
            school_calendar_institution_codes: None,
        };

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt.block_on(aula_api::services::calendar::get_events(
                &mut session,
                &params,
            ))
        };

        match result {
            Ok(events) => {
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                for event in &events {
                    self.insert_calendar_event(&mut inodes, parent_ino, event);
                }

                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&events).unwrap_or_else(|e| {
                        warn!("Failed to serialize events to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    LIST_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch calendar events: {}", e);
            }
        }
    }

    fn insert_calendar_event(
        &self,
        inodes: &mut InodeTable,
        parent_ino: u64,
        event: &EventSimpleDto,
    ) {
        let id = event.id.unwrap_or(0) as i64;
        let title = event.title.as_deref().unwrap_or("untitled");
        let name = dir_name(id, title);

        let ctime = parse_aula_datetime(event.start_date_time.as_deref().unwrap_or(""));
        let mtime = ctime;

        let item_ino = inodes.insert(
            parent_ino,
            name.clone(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Calendar,
                name,
                created: ctime,
                modified: mtime,
            },
        );

        // Details as text.
        let start = event.start_date_time.as_deref().unwrap_or("?");
        let end = event.end_date_time.as_deref().unwrap_or("?");
        let etype = event.event_type.as_deref().unwrap_or("event");
        let all_day = event.all_day.unwrap_or(false);
        let text = format!(
            "Title: {}\nType: {}\nStart: {}\nEnd: {}\nAll day: {}",
            title, etype, start, end, all_day
        );
        let text_bytes = text.len() as u64;
        inodes.insert(
            item_ino,
            "details.txt".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "details.txt".to_string(),
                content: ContentSource::Text(text),
                size: text_bytes,
                mtime,
            },
        );

        // metadata.json.
        let json = serde_json::to_string_pretty(event).unwrap_or_else(|e| {
            warn!("Failed to serialize event metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;
        inodes.insert(
            item_ino,
            "metadata.json".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "metadata.json".to_string(),
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );
    }

    fn populate_notifications(&self, parent_ino: u64) {
        let cache_key = CacheKey::ResourceList {
            resource: "notifications".into(),
            page: 0,
        };

        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return;
            }
        }

        let (children_ids, inst_codes) = {
            let session = self.session.lock_or_recover();
            (
                session.children_inst_profile_ids(),
                session.children_institution_codes(),
            )
        };

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt
                .block_on(aula_api::services::notifications::get_notifications(
                    &mut session,
                    &children_ids,
                    &inst_codes,
                ))
        };

        match result {
            Ok(notifications) => {
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                for notif in &notifications {
                    self.insert_notification(&mut inodes, parent_ino, notif);
                }

                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&notifications).unwrap_or_else(|e| {
                        warn!("Failed to serialize notifications to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    LIST_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch notifications: {}", e);
            }
        }
    }

    fn insert_notification(
        &self,
        inodes: &mut InodeTable,
        parent_ino: u64,
        notif: &NotificationItemDto,
    ) {
        let id = notif.notification_id.as_deref().unwrap_or("unknown");
        let filename = format!("{}.json", crate::sanitize::sanitize_name(id));

        let mtime = parse_aula_datetime(notif.triggered.as_deref().unwrap_or(""));
        let json = serde_json::to_string_pretty(notif).unwrap_or_else(|e| {
            warn!("Failed to serialize notification metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;

        inodes.insert(
            parent_ino,
            filename.clone(),
            InodeEntry::File {
                parent_inode: parent_ino,
                name: filename,
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );
    }

    fn populate_gallery(&self, parent_ino: u64, page: i32) {
        let cache_key = CacheKey::ResourceList {
            resource: "gallery".into(),
            page,
        };

        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return;
            }
        }

        let filter = GalleryViewFilter {
            selected_institution_code_for_filter: None,
            album_id: None,
            user_specific_album: None,
            limit: Some(PAGE_SIZE),
            index: Some(page * PAGE_SIZE),
            sort_on: Some("createdAt".to_string()),
            order_direction: Some("desc".to_string()),
            filter_by: None,
        };

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt.block_on(aula_api::services::gallery::get_albums(
                &mut session,
                &filter,
            ))
        };

        match result {
            Ok(albums) => {
                let has_more = albums.len() == PAGE_SIZE as usize;
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                for album in &albums {
                    self.insert_album(&mut inodes, parent_ino, album);
                }
                if has_more {
                    let next_page = page + 1;
                    let page_name = format!("page-{}", next_page + 1);
                    inodes.insert(
                        parent_ino,
                        page_name,
                        InodeEntry::PageDir {
                            resource_type: ResourceType::Gallery,
                            page: next_page,
                            parent_inode: parent_ino,
                        },
                    );
                }

                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&albums).unwrap_or_else(|e| {
                        warn!("Failed to serialize albums to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    LIST_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch gallery albums: {}", e);
            }
        }
    }

    fn insert_album(&self, inodes: &mut InodeTable, parent_ino: u64, album: &AlbumDto) {
        let id = album.id.unwrap_or(0);
        let title = album
            .title
            .as_deref()
            .or(album.name.as_deref())
            .unwrap_or("untitled");
        let name = dir_name(id, title);

        let mtime = parse_aula_datetime(album.creation_date.as_deref().unwrap_or(""));

        let item_ino = inodes.insert(
            parent_ino,
            name.clone(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Gallery,
                name,
                created: mtime,
                modified: mtime,
            },
        );

        // metadata.json.
        let json = serde_json::to_string_pretty(album).unwrap_or_else(|e| {
            warn!("Failed to serialize album metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;
        inodes.insert(
            item_ino,
            "metadata.json".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "metadata.json".to_string(),
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );

        // Description as content.txt if present.
        if let Some(desc) = &album.description {
            if !desc.is_empty() {
                let text_bytes = desc.len() as u64;
                inodes.insert(
                    item_ino,
                    "description.txt".to_string(),
                    InodeEntry::File {
                        parent_inode: item_ino,
                        name: "description.txt".to_string(),
                        content: ContentSource::Text(desc.clone()),
                        size: text_bytes,
                        mtime,
                    },
                );
            }
        }
    }

    fn populate_documents(&self, parent_ino: u64, page: i32) {
        let cache_key = CacheKey::ResourceList {
            resource: "documents".into(),
            page,
        };

        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return;
            }
        }

        let inst_profile_ids = {
            let session = self.session.lock_or_recover();
            session.all_institution_profile_ids()
        };

        let args = GetSecureDocumentsArguments {
            filter_institution_profile_ids: Some(inst_profile_ids),
            filter_regarding_group_ids: None,
            filter_unread: None,
            filter_locked: None,
            filter_journaling_status: None,
            filter_editable: false,
            document_type: None,
            sortings: None,
            index: Some(page * PAGE_SIZE),
            limit: Some(PAGE_SIZE),
            filter_regarding_student_ids: None,
            filter_document_categories: None,
        };

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt
                .block_on(aula_api::services::documents::get_secure_documents(
                    &mut session,
                    &args,
                ))
        };

        match result {
            Ok(doc_result) => {
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                let docs = doc_result.documents.as_deref().unwrap_or(&[]);
                for doc in docs {
                    self.insert_document(&mut inodes, parent_ino, doc);
                }
                let total = doc_result.total_count.unwrap_or(0);
                let fetched_so_far = (page + 1) * PAGE_SIZE;
                if fetched_so_far < total {
                    let next_page = page + 1;
                    let page_name = format!("page-{}", next_page + 1);
                    inodes.insert(
                        parent_ino,
                        page_name,
                        InodeEntry::PageDir {
                            resource_type: ResourceType::Documents,
                            page: next_page,
                            parent_inode: parent_ino,
                        },
                    );
                }

                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&doc_result).unwrap_or_else(|e| {
                        warn!("Failed to serialize documents to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    LIST_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch documents: {}", e);
            }
        }
    }

    fn insert_document(&self, inodes: &mut InodeTable, parent_ino: u64, doc: &SecureDocumentDto) {
        let id = doc.id.unwrap_or(0);
        let title = doc.title.as_deref().unwrap_or("untitled");
        let name = dir_name(id, title);

        let ctime = parse_aula_datetime(doc.created_at.as_deref().unwrap_or(""));
        let mtime = mtime_from(doc.updated_at.as_deref(), doc.created_at.as_deref());

        let item_ino = inodes.insert(
            parent_ino,
            name.clone(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Documents,
                name,
                created: ctime,
                modified: mtime,
            },
        );

        // metadata.json.
        let json = serde_json::to_string_pretty(doc).unwrap_or_else(|e| {
            warn!("Failed to serialize document metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;
        inodes.insert(
            item_ino,
            "metadata.json".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "metadata.json".to_string(),
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );

        // Description as content.txt if present.
        if let Some(desc) = &doc.description {
            if !desc.is_empty() {
                let text_bytes = desc.len() as u64;
                inodes.insert(
                    item_ino,
                    "content.txt".to_string(),
                    InodeEntry::File {
                        parent_inode: item_ino,
                        name: "content.txt".to_string(),
                        content: ContentSource::Text(desc.clone()),
                        size: text_bytes,
                        mtime,
                    },
                );
            }
        }
    }

    fn populate_presence(&self, parent_ino: u64) {
        let cache_key = CacheKey::ResourceList {
            resource: "presence".into(),
            page: 0,
        };

        {
            let cache = self.cache.lock_or_recover();
            if cache.get(&cache_key).is_some() {
                return;
            }
        }

        let children_ids = {
            let session = self.session.lock_or_recover();
            session.children_inst_profile_ids()
        };

        if children_ids.is_empty() {
            return;
        }

        let result = {
            let mut session = self.session.lock_or_recover();
            self.rt
                .block_on(aula_api::services::presence::get_childrens_state(
                    &mut session,
                    &children_ids,
                ))
        };

        match result {
            Ok(statuses) => {
                let mut inodes = self.inodes.lock_or_recover();
                inodes.clear_children(parent_ino);
                for status in &statuses {
                    self.insert_presence(&mut inodes, parent_ino, status);
                }

                let mut cache = self.cache.lock_or_recover();
                cache.put(
                    cache_key,
                    serde_json::to_value(&statuses).unwrap_or_else(|e| {
                        warn!("Failed to serialize presence statuses to cache value: {e}");
                        serde_json::Value::Null
                    }),
                    PRESENCE_TTL,
                );
            }
            Err(e) => {
                error!("Failed to fetch presence: {}", e);
            }
        }
    }

    fn insert_presence(&self, inodes: &mut InodeTable, parent_ino: u64, status: &ChildStatusDto) {
        let child_name = status
            .uni_student
            .as_ref()
            .and_then(|s| s.name.as_deref())
            .unwrap_or("unknown");
        let name = crate::sanitize::sanitize_name(child_name);

        let mtime = SystemTime::now();

        let item_ino = inodes.insert(
            parent_ino,
            name.clone(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Presence,
                name,
                created: mtime,
                modified: mtime,
            },
        );

        // Status as text.
        let status_str = status
            .state
            .as_ref()
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "unknown".to_string());
        let text = format!("Status: {}", status_str);
        let text_bytes = text.len() as u64;
        inodes.insert(
            item_ino,
            "status.txt".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "status.txt".to_string(),
                content: ContentSource::Text(text),
                size: text_bytes,
                mtime,
            },
        );

        // metadata.json.
        let json = serde_json::to_string_pretty(status).unwrap_or_else(|e| {
            warn!("Failed to serialize presence status metadata: {e}");
            String::new()
        });
        let json_bytes = json.len() as u64;
        inodes.insert(
            item_ino,
            "metadata.json".to_string(),
            InodeEntry::File {
                parent_inode: item_ino,
                name: "metadata.json".to_string(),
                content: ContentSource::Text(json),
                size: json_bytes,
                mtime,
            },
        );
    }
}

/// Minimal HTML tag stripping (good enough for display purposes).
fn strip_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut tag_name = String::new();
    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag = true;
                tag_name.clear();
            }
            '>' => {
                in_tag = false;
                let is_closing = tag_name.starts_with('/');
                let name = tag_name.trim_start_matches('/').to_ascii_lowercase();
                // Void elements: insert newline on the (only) tag.
                // Paired block elements: insert newline only on closing tag.
                let newline = match name.as_str() {
                    "br" => true,
                    "p" | "div" | "li" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "tr" => {
                        is_closing
                    }
                    _ => false,
                };
                if newline {
                    result.push('\n');
                }
            }
            _ if in_tag => {
                // Accumulate tag name (stop at space/attrs).
                if ch == ' ' || ch == '\t' || ch == '\n' || ch == '/' && !tag_name.is_empty() {
                    // Attributes follow; stop accumulating tag name.
                } else if tag_name.len() < 10 {
                    tag_name.push(ch);
                }
            }
            _ => result.push(ch),
        }
    }
    result.trim().to_string()
}

// =============================================================================
// Filesystem trait implementation
// =============================================================================

impl Filesystem for AulaFs {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let name_str = match name.to_str() {
            Some(s) => s,
            None => {
                reply.error(libc::ENOENT);
                return;
            }
        };

        debug!("lookup: parent={}, name={}", parent, name_str);

        // First, try to populate if this is a resource dir that hasn't been filled yet.
        {
            let inodes = self.inodes.lock_or_recover();
            if let Some(entry) = inodes.get(parent) {
                match entry {
                    InodeEntry::ResourceDir(rt) => {
                        let rt = *rt;
                        drop(inodes);
                        self.populate_resource_dir(parent, rt, 0);
                    }
                    InodeEntry::PageDir {
                        resource_type,
                        page,
                        ..
                    } => {
                        let rt = *resource_type;
                        let pg = *page;
                        drop(inodes);
                        self.populate_resource_dir(parent, rt, pg);
                    }
                    _ => {}
                }
            }
        }

        let inodes = self.inodes.lock_or_recover();
        if let Some(ino) = inodes.lookup(parent, name_str) {
            if let Some(entry) = inodes.get(ino) {
                let attr = self.attr_for(ino, entry);
                reply.entry(&FUSE_TTL, &attr, 0);
                return;
            }
        }

        reply.error(libc::ENOENT);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        debug!("getattr: ino={}", ino);

        let inodes = self.inodes.lock_or_recover();
        if let Some(entry) = inodes.get(ino) {
            let attr = self.attr_for(ino, entry);
            reply.attr(&FUSE_TTL, &attr);
        } else {
            reply.error(libc::ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        debug!("readdir: ino={}, offset={}", ino, offset);

        // Populate lazily if needed.
        {
            let inodes = self.inodes.lock_or_recover();
            if let Some(entry) = inodes.get(ino) {
                match entry {
                    InodeEntry::ResourceDir(rt) if !inodes.has_children(ino) => {
                        let rt = *rt;
                        drop(inodes);
                        self.populate_resource_dir(ino, rt, 0);
                    }
                    InodeEntry::PageDir {
                        resource_type,
                        page,
                        ..
                    } if !inodes.has_children(ino) => {
                        let rt = *resource_type;
                        let pg = *page;
                        drop(inodes);
                        self.populate_resource_dir(ino, rt, pg);
                    }
                    _ => {}
                }
            }
        }

        let inodes = self.inodes.lock_or_recover();

        let mut entries: Vec<(String, u64, FileType)> = Vec::new();

        // Add . and ..
        entries.push((".".to_string(), ino, FileType::Directory));
        let parent_ino = inodes.parent_of(ino);
        entries.push(("..".to_string(), parent_ino, FileType::Directory));

        // Add actual children.
        let children = inodes.readdir(ino);
        for (name, child_ino) in children {
            let file_type = match inodes.get(child_ino) {
                Some(InodeEntry::File { .. }) => FileType::RegularFile,
                _ => FileType::Directory,
            };
            entries.push((name, child_ino, file_type));
        }

        // Apply offset.
        for (i, (name, child_ino, file_type)) in entries.iter().enumerate().skip(offset as usize) {
            // Reply returns true if the buffer is full.
            if reply.add(*child_ino, (i + 1) as i64, *file_type, name) {
                break;
            }
        }

        reply.ok();
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        debug!("read: ino={}, offset={}, size={}", ino, offset, size);

        let inodes = self.inodes.lock_or_recover();
        if let Some(InodeEntry::File { content, .. }) = inodes.get(ino) {
            let bytes: Vec<u8> = match content {
                ContentSource::Text(t) => t.as_bytes().to_vec(),
                ContentSource::LazyDownload { url } => {
                    // Lazy download: fetch the content from the URL.
                    let url = url.clone();
                    drop(inodes);
                    match self.lazy_download(&url) {
                        Ok(data) => data,
                        Err(e) => {
                            error!("Lazy download failed for {}: {}", url, e);
                            reply.error(libc::EIO);
                            return;
                        }
                    }
                }
                ContentSource::Empty => Vec::new(),
            };

            let offset = offset as usize;
            if offset >= bytes.len() {
                reply.data(&[]);
            } else {
                let end = std::cmp::min(offset + size as usize, bytes.len());
                reply.data(&bytes[offset..end]);
            }
        } else {
            reply.error(libc::ENOENT);
        }
    }

    // Read-only filesystem: reject all write operations.

    fn write(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    ) {
        reply.error(libc::EROFS);
    }

    fn mkdir(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _umask: u32,
        reply: ReplyEntry,
    ) {
        reply.error(libc::EROFS);
    }

    fn unlink(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: fuser::ReplyEmpty) {
        reply.error(libc::EROFS);
    }

    fn rmdir(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: fuser::ReplyEmpty) {
        reply.error(libc::EROFS);
    }

    fn create(
        &mut self,
        _req: &Request,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _umask: u32,
        _flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        reply.error(libc::EROFS);
    }

    fn setattr(
        &mut self,
        _req: &Request,
        _ino: u64,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        _size: Option<u64>,
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        // For setattr, return current attributes (read-only, no changes).
        let inodes = self.inodes.lock_or_recover();
        if let Some(entry) = inodes.get(_ino) {
            let attr = self.attr_for(_ino, entry);
            reply.attr(&FUSE_TTL, &attr);
        } else {
            reply.error(libc::ENOENT);
        }
    }
}

impl AulaFs {
    fn lazy_download(&self, url: &str) -> Result<Vec<u8>, String> {
        // Use the session's HTTP client to download binary content.
        let client = {
            let session = self.session.lock_or_recover();
            session.client().http().clone()
        };

        self.rt.block_on(async {
            let resp = client
                .get(url)
                .send()
                .await
                .map_err(|e| format!("HTTP error: {}", e))?;
            resp.bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| format!("Body error: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_html_basic() {
        assert_eq!(strip_html("<b>hello</b>"), "hello");
        assert_eq!(strip_html("<p>one</p><p>two</p>"), "one\ntwo");
        assert_eq!(strip_html("no tags here"), "no tags here");
        assert_eq!(strip_html(""), "");
    }

    #[test]
    fn strip_html_no_spurious_newlines_from_text() {
        // Text ending in 'p', 'br', 'div', 'li' must NOT trigger newlines.
        assert_eq!(strip_html("trip"), "trip");
        assert_eq!(strip_html("grip"), "grip");
        assert_eq!(strip_html("stop"), "stop");
        assert_eq!(strip_html("abr"), "abr");
        assert_eq!(strip_html("adiv"), "adiv");
        assert_eq!(strip_html("ali"), "ali");
        // Inside tags, same.
        assert_eq!(strip_html("<span>trip</span>"), "trip");
        assert_eq!(strip_html("<span>We went on a trip</span>"), "We went on a trip");
    }

    #[test]
    fn strip_html_block_tags_produce_newlines() {
        assert_eq!(strip_html("<p>para</p>"), "para");
        assert_eq!(strip_html("line1<br>line2"), "line1\nline2");
        assert_eq!(strip_html("line1<br/>line2"), "line1\nline2");
        assert_eq!(strip_html("<div>block</div>"), "block");
        assert_eq!(strip_html("<li>item</li>"), "item");
        assert_eq!(strip_html("<h1>heading</h1>"), "heading");
        assert_eq!(strip_html("<tr>row</tr>"), "row");
    }

    #[test]
    fn strip_html_mixed_content() {
        let input = "<p>We went on a <b>trip</b> today.</p><p>It was great!</p>";
        assert_eq!(strip_html(input), "We went on a trip today.\nIt was great!");
    }
}
