//! Tool registry: names, JSON schemas, and handlers.
//!
//! Every handler projects Aula's nested DTOs down to the fields a question can
//! actually be answered from (see [`crate::compact`]). Reads are safe; the two
//! write tools send real messages, so they validate their arguments and echo
//! back exactly what was sent.

use aula_api::models::calendar::GetEventsParameters;
use aula_api::models::documents::GetSecureDocumentsArguments;
use aula_api::models::gallery::{GalleryViewFilter, GetMediaInAlbumFilter};
use aula_api::models::messaging::{
    GetMessagesForThreadArguments, GetThreadListArguments, MessageContentRequest,
    RecipientApiModel, ReplyMessageArgument, StartNewThreadRequestArguments,
};
use aula_api::models::posts::GetPostApiParameters;
use aula_api::models::search::SearchForProfilesAndGroupsParameters;
use aula_api::models::search::{GlobalSearchParameters, SearchGroupRequestModel};
use aula_api::services::{
    calendar, documents, gallery, messaging, notifications, posts, presence, search,
};
use aula_api::{Environment, Session, TokenStore};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::compact::{name_of, non_null, pick, text, to_value};

/// Default page size for list tools.
const DEFAULT_LIMIT: i32 = 20;

/// Maximum characters of a message body returned by `aula_thread`.
const BODY_LIMIT: usize = 4000;

/// Shared server state: one long-lived session.
///
/// `ensure_context_initialized()` costs two HTTP round-trips, so the session is
/// created once and reused across tool calls.
pub struct Server {
    session: Mutex<Option<Session>>,
    /// Store the session is built on. `None` means the default on-disk store;
    /// a hosted transport supplies its own (e.g. tokens fetched from Supabase,
    /// wrapped in a `TokenStore::in_memory`).
    store: Option<TokenStore>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(None),
            store: None,
        }
    }

    /// Build a server whose session reads and writes tokens through `store`.
    ///
    /// The caller keeps its own clone of the store to observe refreshed tokens
    /// afterwards and persist them wherever it likes.
    pub fn with_store(store: TokenStore) -> Self {
        Self {
            session: Mutex::new(None),
            store: Some(store),
        }
    }

    /// Lock the session, creating it on first use.
    async fn lock(&self) -> Result<tokio::sync::MutexGuard<'_, Option<Session>>, String> {
        let mut guard = self.session.lock().await;
        if guard.is_none() {
            let name = std::env::var("AULA_ENV").unwrap_or_else(|_| "production".to_string());
            let environment = Environment::from_name(&name);
            let store = match self.store.clone() {
                Some(store) => store,
                None => {
                    TokenStore::default_location().unwrap_or_else(|| TokenStore::new("./aula-data"))
                }
            };
            let store_kind = store
                .token_path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "in-memory tokens".to_string());

            let session = aula_api::session::from_store_with(environment, store).map_err(|e| {
                format!("cannot open an Aula session ({e}); token store: {store_kind}")
            })?;

            // No tokens at all means every call would come back as a bare 403
            // from Aula - say what is actually wrong instead.
            let Some(login_data) = session.login_data() else {
                eprintln!("[aula-mcp] no tokens found ({store_kind})");
                return Err(format!(
                    "not logged in: no Aula tokens in {store_kind}. \
                     Run `aula auth login`, or set AULA_DATA_DIR to the directory \
                     that contains tokens.json."
                ));
            };
            eprintln!(
                "[aula-mcp] tokens loaded from {store_kind} (expires {:?}), env={name}",
                login_data.access_token_expiration
            );

            *guard = Some(session);
        }
        Ok(guard)
    }
}

// ---------------------------------------------------------------------------
// Argument helpers
// ---------------------------------------------------------------------------

fn arg_i64(args: &Value, key: &str) -> Option<i64> {
    args.get(key).and_then(Value::as_i64)
}

fn arg_i32(args: &Value, key: &str) -> Option<i32> {
    arg_i64(args, key).map(|v| v as i32)
}

fn arg_str(args: &Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn arg_bool(args: &Value, key: &str) -> Option<bool> {
    args.get(key).and_then(Value::as_bool)
}

fn arg_i64_array(args: &Value, key: &str) -> Option<Vec<i64>> {
    args.get(key)
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_i64).collect::<Vec<i64>>())
}

fn api_err(e: aula_api::AulaError) -> String {
    e.to_string()
}

/// Institution profile IDs for every profile in the session (guardian + children).
async fn all_ids(session: &mut Session) -> Result<Vec<i64>, String> {
    session
        .ensure_context_initialized()
        .await
        .map_err(api_err)?;
    Ok(session.all_institution_profile_ids())
}

// ---------------------------------------------------------------------------
// Tool listing
// ---------------------------------------------------------------------------

fn schema(properties: Value, required: &[&str]) -> Value {
    json!({"type": "object", "properties": properties, "required": required})
}

fn num(description: &str) -> Value {
    json!({"type": "integer", "description": description})
}

fn text_arg(description: &str) -> Value {
    json!({"type": "string", "description": description})
}

fn flag(description: &str) -> Value {
    json!({"type": "boolean", "description": description})
}

/// Tool descriptors returned by `tools/list`.
pub fn list() -> Value {
    json!([
        {
            "name": "aula_whoami",
            "description": "Who the logged-in guardian is: own profile, institutions, and children. Also returns the institution profile IDs that other tools accept (recipients, child filters). Call this first when you need a child's name or an ID.",
            "inputSchema": schema(json!({}), &[]),
        },
        {
            "name": "aula_messages",
            "description": "List message threads from the Aula inbox (newest first) with sender, timestamp, unread flag and a short preview. Use for 'what has the school sent me', 'anything unread'. Aula omits the sender in list responses, so use aula_thread to see who wrote which message.",
            "inputSchema": schema(json!({
                "limit": num("Maximum threads to return (default 20)."),
                "unread_only": flag("Only threads with unread messages."),
                "folder_id": num("Restrict to one message folder (see folder ids from the CLI)."),
            }), &[]),
        },
        {
            "name": "aula_thread",
            "description": "Read one message thread: subject, participants and the message bodies in plain text. Use after aula_messages to read what was actually said. Long bodies are truncated; paginate with `page` for older messages.",
            "inputSchema": schema(json!({
                "thread_id": num("Thread id from aula_messages."),
                "page": num("Page of messages, 0 = most recent (default 0)."),
            }), &["thread_id"]),
        },
        {
            "name": "aula_calendar",
            "description": "Calendar events for a date range across the guardian and all children: school-home meetings, holidays, deadlines, events requiring a response. Timetable lessons are excluded unless include_lessons is set.",
            "inputSchema": schema(json!({
                "from": text_arg("Start date, YYYY-MM-DD (default: today)."),
                "to": text_arg("End date, YYYY-MM-DD (default: 7 days after start)."),
                "include_lessons": flag("Include the daily timetable (type 'lesson'). Default false, since it is mostly noise."),
                "limit": num("Maximum events to return (default 100)."),
            }), &[]),
        },
        {
            "name": "aula_presence",
            "description": "Current presence state per child (at school, sick, absent, etc.). Use for 'is my child at school today'.",
            "inputSchema": schema(json!({}), &[]),
        },
        {
            "name": "aula_posts",
            "description": "Posts from the institution feed (class/school news), with author, timestamp and plain-text body.",
            "inputSchema": schema(json!({
                "limit": num("Maximum posts to return (default 20)."),
            }), &[]),
        },
        {
            "name": "aula_notifications",
            "description": "Notification feed: new messages, calendar changes, documents, gallery uploads, pickup changes. Each item carries notificationEventType (what happened) plus links back to the underlying ids. Badge-type notifications have no text by design.",
            "inputSchema": schema(json!({
                "limit": num("Maximum notifications to return (default 20)."),
            }), &[]),
        },
        {
            "name": "aula_gallery",
            "description": "Photo albums shared with guardians, or the media inside one album (with image URLs). Call without album_id to list albums.",
            "inputSchema": schema(json!({
                "album_id": num("Album id to list media for. Omit to list albums."),
                "limit": num("Maximum items to return (default 20)."),
            }), &[]),
        },
        {
            "name": "aula_documents",
            "description": "Secure documents shared with guardians (school letters, plans, evaluations) with title, category and dates. Aula answers 403 if the school does not use secure documents - report that instead of retrying.",
            "inputSchema": schema(json!({
                "limit": num("Maximum documents to return (default 20)."),
            }), &[]),
        },
        {
            "name": "aula_search",
            "description": "Search Aula for people and groups (profiles, classes, institutions). Use it to resolve a name into an id. Full-text content search is unavailable: Aula's findGeneric endpoint fails server-side, so this falls back to profile and group search.",
            "inputSchema": schema(json!({
                "query": text_arg("Text to search for, e.g. a teacher's name."),
                "limit": num("Maximum results to return (default 10)."),
            }), &["query"]),
        },
        {
            "name": "aula_send_message",
            "description": "WRITE: send a new message as you, to real recipients on your children's school platform. Recipients must be institution profile ids from aula_whoami or aula_search. The result echoes the recipients and subject that were actually sent - check them.",
            "inputSchema": schema(json!({
                "recipients": json!({
                    "type": "array",
                    "items": {"type": "integer"},
                    "description": "Institution profile ids to send to (not profile ids).",
                }),
                "subject": text_arg("Message subject."),
                "body": text_arg("Message body, plain text."),
            }), &["recipients", "body"]),
        },
        {
            "name": "aula_reply",
            "description": "WRITE: reply to an existing message thread as you, visible to all its participants. The result echoes the thread id and body that were sent - check them.",
            "inputSchema": schema(json!({
                "thread_id": num("Thread id from aula_messages."),
                "body": text_arg("Reply body, plain text."),
            }), &["thread_id", "body"]),
        },
    ])
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Run a tool call, returning an MCP `CallToolResult`.
///
/// Protocol-level problems (unknown tool, missing name) are `Err((code, msg))`;
/// tool execution failures become `isError: true` content so the model can see
/// and recover from them.
pub async fn call(server: &Server, params: &Value) -> Result<Value, (i64, String)> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or((-32602, "tools/call requires a tool name".to_string()))?;
    let args = params.get("arguments").cloned().unwrap_or(Value::Null);

    let outcome = match name {
        "aula_whoami" => whoami(server).await,
        "aula_messages" => messages(server, &args).await,
        "aula_thread" => thread(server, &args).await,
        "aula_calendar" => calendar_events(server, &args).await,
        "aula_presence" => presence_now(server).await,
        "aula_posts" => feed_posts(server, &args).await,
        "aula_notifications" => notification_feed(server, &args).await,
        "aula_gallery" => gallery_items(server, &args).await,
        "aula_documents" => secure_documents(server, &args).await,
        "aula_search" => search_aula(server, &args).await,
        "aula_send_message" => send_message(server, &args).await,
        "aula_reply" => reply(server, &args).await,
        other => return Err((-32602, format!("unknown tool: {other}"))),
    };

    Ok(match outcome {
        Ok(value) => json!({
            "content": [{"type": "text", "text": pretty(&non_null(value))}],
            "isError": false,
        }),
        Err(message) => json!({
            "content": [{"type": "text", "text": message}],
            "isError": true,
        }),
    })
}

fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}

// ---------------------------------------------------------------------------
// Handlers: reads
// ---------------------------------------------------------------------------

async fn whoami(server: &Server) -> Result<Value, String> {
    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    session
        .ensure_context_initialized()
        .await
        .map_err(api_err)?;

    let profile_data = session
        .profile_data()
        .ok_or("Aula returned no profile data for this login")?;

    let mut profiles = Vec::new();
    let mut children = Vec::new();
    for profile in &profile_data.profiles {
        profiles.push(json!({
            "name": profile.display_name,
            "portal_role": profile.portal_role,
            "institutions": profile
                .institution_profiles
                .as_ref()
                .map(|institutions| institutions
                    .iter()
                    .map(|i| json!({
                        "institution": i.institution_name,
                        "institution_profile_id": i.id,
                        "institution_code": i.institution_code,
                        "role": i.role,
                    }))
                    .collect::<Vec<_>>())
                .unwrap_or_default(),
        }));
        for child in profile.children.as_deref().unwrap_or_default() {
            children.push(json!({
                "name": child.name,
                "profile_id": child.profile_id,
                "institution_profile_id": child.institution_profile.as_ref().and_then(|i| i.id),
                "institution_code": child.institution_code,
            }));
        }
    }

    Ok(json!({
        "profiles": profiles,
        "children": children,
        "institution_profile_ids": session.institution_profile_ids(),
        "children_institution_profile_ids": session.children_inst_profile_ids(),
        "children_institution_codes": session.children_institution_codes(),
    }))
}

async fn messages(server: &Server, args: &Value) -> Result<Value, String> {
    let limit = arg_i32(args, "limit").unwrap_or(DEFAULT_LIMIT).max(1) as usize;
    let unread_only = arg_bool(args, "unread_only").unwrap_or(false);
    let folder_id = arg_i64(args, "folder_id");

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let list = messaging::get_thread_list(
        session,
        &GetThreadListArguments {
            folder_id,
            filter_type: None,
            sort_type: None,
            sort_order: None,
            page: Some(0),
            thread_ids: None,
            mail_box_owner_type: None,
            mail_box_owners: None,
            active_children: None,
        },
    )
    .await
    .map_err(api_err)?;

    let threads = list
        .threads
        .unwrap_or_default()
        .into_iter()
        .filter(|t| !unread_only || !t.read)
        .take(limit)
        .map(|t| {
            let latest = t.latest_message.as_ref();
            json!({
                "thread_id": t.id,
                "subject": t.subject,
                "unread": !t.read,
                "marked": t.marked,
                "last_message_at": latest.and_then(|m| m.send_date_time.clone()),
                "from": latest
                    .and_then(|m| m.sender.as_ref())
                    .and_then(|box_| box_.display_name.clone()),
                "preview": latest
                    .and_then(|m| m.text.as_ref())
                    .and_then(|rich| text(rich.html.as_deref(), 240)),
                "has_attachments": latest.map(|m| m.has_attachments).unwrap_or(false),
                "recipients": t
                    .recipients
                    .as_deref()
                    .unwrap_or_default()
                    .iter()
                    .filter_map(name_of)
                    .collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({"count": threads.len(), "threads": threads}))
}

async fn thread(server: &Server, args: &Value) -> Result<Value, String> {
    let thread_id = arg_i64(args, "thread_id").ok_or("thread_id is required")?;
    let page = arg_i32(args, "page").unwrap_or(0);

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let dto = messaging::get_thread_by_id(
        session,
        &GetMessagesForThreadArguments {
            thread_id: Some(thread_id),
            page: Some(page),
            common_inbox_id: None,
        },
    )
    .await
    .map_err(api_err)?;

    let messages = dto
        .messages
        .unwrap_or_default()
        .iter()
        .map(|m| {
            json!({
                "id": m.id,
                "from": m
                    .sender
                    .as_ref()
                    .and_then(name_of),
                "sent_at": m.send_date_time,
                "body": m.text.as_ref().and_then(|rich| text(rich.html.as_deref(), BODY_LIMIT)),
                "attachment_count": m.attachments.as_ref().map(Vec::len).unwrap_or(0),
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "thread_id": dto.id,
        "subject": dto.subject,
        "page": dto.page,
        "total_messages": dto.total_message_count,
        "more_messages_exist": dto.more_messages_exist,
        "participants": dto
            .recipients
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(name_of)
            .collect::<Vec<_>>(),
        "messages": messages,
    }))
}

async fn calendar_events(server: &Server, args: &Value) -> Result<Value, String> {
    let today = chrono::Local::now().date_naive();
    let from = arg_str(args, "from").unwrap_or_else(|| today.format("%Y-%m-%d").to_string());
    let to = arg_str(args, "to").unwrap_or_else(|| {
        (today + chrono::Duration::days(7))
            .format("%Y-%m-%d")
            .to_string()
    });
    let include_lessons = arg_bool(args, "include_lessons").unwrap_or(false);
    let limit = arg_i32(args, "limit").unwrap_or(100).max(1);

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let inst_profile_ids = all_ids(session).await?;
    let events = calendar::get_events(
        session,
        &GetEventsParameters {
            inst_profile_ids: if inst_profile_ids.is_empty() {
                None
            } else {
                Some(inst_profile_ids)
            },
            resource_ids: None,
            start: Some(from.clone()),
            end: Some(to.clone()),
            specific_types: None,
            school_calendar_institution_codes: None,
        },
    )
    .await
    .map_err(api_err)?;

    let items = events
        .iter()
        .filter(|e| include_lessons || e.event_type.as_deref() != Some("lesson"))
        .map(|e| {
            json!({
                "id": e.id,
                "title": e.title,
                "start": e.start_date_time,
                "end": e.end_date_time,
                "all_day": e.all_day,
                "type": e.event_type,
                "location": e.primary_resource_text,
                "requires_response": e.response_required,
                "for_profiles": e.belongs_to_profiles,
            })
        })
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(json!({"from": from, "to": to, "count": items.len(), "events": items}))
}

async fn presence_now(server: &Server) -> Result<Value, String> {
    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    session
        .ensure_context_initialized()
        .await
        .map_err(api_err)?;
    let child_ids = session.children_inst_profile_ids();
    if child_ids.is_empty() {
        return Ok(json!({"children": [], "note": "no children on this login"}));
    }

    let states = presence::get_childrens_state(session, &child_ids)
        .await
        .map_err(api_err)?;

    let children = states
        .iter()
        .map(|c| {
            json!({
                "child": c.uni_student.as_ref().and_then(|s| s.name.clone()),
                "state": c.state.as_ref().map(|s| format!("{s:?}")),
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({"children": children}))
}

async fn feed_posts(server: &Server, args: &Value) -> Result<Value, String> {
    let limit = arg_i32(args, "limit").unwrap_or(DEFAULT_LIMIT).max(1);

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let ids = all_ids(session).await?;
    let result = posts::get_posts(
        session,
        &GetPostApiParameters {
            parent: None,
            group_id: None,
            is_important: None,
            creator_portal_role: None,
            institution_profile_ids: if ids.is_empty() { None } else { Some(ids) },
            related_institutions: None,
            own_post: false,
            is_unread: false,
            is_bookmarked: false,
            limit: Some(limit),
            index: Some(0),
        },
    )
    .await
    .map_err(api_err)?;

    let items = result
        .posts
        .unwrap_or_default()
        .iter()
        .map(|p| {
            json!({
                "id": p.id,
                "title": p.title,
                "author": p.owner_profile.as_ref().and_then(name_of),
                "published_at": p.time_stamp.clone().or_else(|| p.publish_at.clone()),
                "body": p.content.as_ref().and_then(|c| text(c.html.as_deref(), 2000)),
                "comment_count": p.comment_count,
                "important": p.is_important,
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({"count": items.len(), "posts": items}))
}

async fn notification_feed(server: &Server, args: &Value) -> Result<Value, String> {
    let limit = arg_i32(args, "limit").unwrap_or(DEFAULT_LIMIT).max(1) as usize;

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    session
        .ensure_context_initialized()
        .await
        .map_err(api_err)?;
    let child_ids = session.children_inst_profile_ids();
    let codes = session.children_institution_codes();

    let items = notifications::get_notifications(session, &child_ids, &codes)
        .await
        .map_err(api_err)?;

    let projected = items
        .iter()
        .take(limit)
        .map(|n| {
            let raw = to_value(n);
            // Aula notifications carry ~60 mostly-null fields whose populated
            // subset depends on the event type, so pick the informative ones
            // rather than mirroring the DTO. "Badge" notifications (e.g.
            // MediaAddedToAlbum) have no text at all - only the linked id/name.
            let mut out = pick(
                &raw,
                &[
                    "notificationEventType",
                    "notificationType",
                    "triggered",
                    "title",
                    "originalTitle",
                    "senderName",
                    "relatedChildName",
                    "postTitle",
                    "albumName",
                    "groupName",
                    "commonInboxName",
                    "roomName",
                    "threadId",
                    "postId",
                    "albumId",
                    "documentId",
                    "eventId",
                    "folderId",
                    "mediaId",
                    "url",
                    "responseDeadline",
                ],
            );
            let body = [
                "messageText",
                "message",
                "notificationMessage",
                "noteToGuardians",
            ]
            .iter()
            .find_map(|k| raw.get(*k).and_then(Value::as_str))
            .and_then(|t| text(Some(t), 500))
            .or_else(|| {
                raw.get("content")
                    .and_then(|c| c.get("html"))
                    .and_then(Value::as_str)
                    .and_then(|t| text(Some(t), 500))
            });
            if let Some(body) = body {
                out["body"] = json!(body);
            }
            out
        })
        .collect::<Vec<_>>();

    Ok(json!({"count": projected.len(), "notifications": projected}))
}

async fn gallery_items(server: &Server, args: &Value) -> Result<Value, String> {
    let limit = arg_i32(args, "limit").unwrap_or(DEFAULT_LIMIT).max(1);
    let album_id = arg_i64(args, "album_id");

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");

    match album_id {
        Some(album_id) => {
            let result = gallery::get_medias_in_album(
                session,
                &GetMediaInAlbumFilter {
                    album_id: Some(album_id),
                    user_specific_album: Some(false),
                    limit: Some(limit),
                    index: Some(0),
                    sort_on: None,
                    order_direction: None,
                    filter_by: None,
                    is_selection_mode: false,
                    selected_institution_code: None,
                },
            )
            .await
            .map_err(api_err)?;

            let media = result
                .results
                .unwrap_or_default()
                .iter()
                .map(|m| {
                    json!({
                        "id": m.id,
                        "title": m.title,
                        "type": m.media_type,
                        "thumbnail_url": m.thumbnail_url,
                        "url": m.file.as_ref().and_then(|f| f.url.clone()),
                        "created": m.file.as_ref().and_then(|f| f.created.clone()),
                    })
                })
                .collect::<Vec<_>>();

            Ok(json!({
                "album_id": album_id,
                "album": result.album.as_ref().and_then(|a| a.title.clone().or_else(|| a.name.clone())),
                "count": media.len(),
                "media": media,
            }))
        }
        None => {
            let albums = gallery::get_albums(
                session,
                &GalleryViewFilter {
                    selected_institution_code_for_filter: None,
                    album_id: None,
                    user_specific_album: Some(false),
                    limit: Some(limit),
                    index: Some(0),
                    sort_on: None,
                    order_direction: None,
                    filter_by: None,
                },
            )
            .await
            .map_err(api_err)?;

            let items = albums
                .iter()
                .map(|a| {
                    json!({
                        "album_id": a.id,
                        "title": a.title.clone().or_else(|| a.name.clone()),
                        "created": a.creation_date,
                        "description": a.description,
                        "media_count": a.size.or(a.total_size),
                        "creator": a.creator.as_ref().and_then(name_of),
                    })
                })
                .collect::<Vec<_>>();

            Ok(json!({"count": items.len(), "albums": items}))
        }
    }
}

async fn secure_documents(server: &Server, args: &Value) -> Result<Value, String> {
    let limit = arg_i32(args, "limit").unwrap_or(DEFAULT_LIMIT).max(1);

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let ids = all_ids(session).await?;
    let result = documents::get_secure_documents(
        session,
        &GetSecureDocumentsArguments {
            filter_institution_profile_ids: if ids.is_empty() { None } else { Some(ids) },
            filter_regarding_group_ids: None,
            filter_unread: None,
            filter_locked: None,
            filter_journaling_status: None,
            filter_editable: false,
            document_type: None,
            sortings: None,
            index: Some(0),
            limit: Some(limit),
            filter_regarding_student_ids: None,
            filter_document_categories: None,
        },
    )
    .await
    .map_err(api_err)?;

    let items = result
        .documents
        .unwrap_or_default()
        .iter()
        .map(|d| {
            json!({
                "id": d.id,
                "title": d.title,
                "category": d.category,
                "type": d.document_type,
                "created_at": d.created_at,
                "updated_at": d.updated_at,
                "locked": d.is_locked,
                "creator": d.creator.as_ref().and_then(name_of),
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({"count": items.len(), "documents": items}))
}

async fn search_aula(server: &Server, args: &Value) -> Result<Value, String> {
    let query = arg_str(args, "query").ok_or("query is required")?;
    let limit = arg_i32(args, "limit").unwrap_or(10).max(1);

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");

    // `search.findGeneric` is broken server-side (HTTP 500 "intern fejl") for
    // guardian accounts, so fall back to findProfiles + findGroups the way
    // aula-cli does.
    let mut results = Vec::new();
    let mut source = "findGeneric";
    match search::global_search(
        session,
        &GlobalSearchParameters {
            text: Some(query.clone()),
            page_limit: Some(limit),
            page_number: None,
            group_id: None,
            doc_type_count: false,
            doc_type: None,
            group_types: None,
        },
    )
    .await
    {
        Ok(response) => results.extend(project_results(&response)),
        Err(_) => {
            source = "findProfiles+findGroups";
            let profiles = search::search_for_profiles(
                session,
                &SearchForProfilesAndGroupsParameters {
                    only_profiles: false,
                    text: Some(query.clone()),
                    portal_roles: None,
                    typeahead: false,
                    limit: Some(limit),
                },
            )
            .await
            .map_err(api_err)?;
            results.extend(project_results(&profiles));

            let groups = search::search_groups(
                session,
                &SearchGroupRequestModel {
                    text: Some(query.clone()),
                    institution_codes: None,
                    limit: Some(limit),
                    offset: Some(0),
                    from_module_value: None,
                },
            )
            .await;
            if let Ok(groups) = groups {
                results.extend(groups.results.unwrap_or_default().iter().map(|g| {
                    pick(
                        &to_value(g),
                        &["id", "name", "institutionName", "institutionCode"],
                    )
                }));
            }
        }
    }

    Ok(json!({
        "query": query,
        "source": source,
        "count": results.len(),
        "results": results,
    }))
}

/// Compact projection of search hits (Aula's item type varies by docType).
fn project_results(response: &aula_api::models::search::SearchResponse) -> Vec<Value> {
    response
        .results
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|r| {
            pick(
                &to_value(r),
                &[
                    "docId",
                    "docType",
                    "name",
                    "title",
                    "description",
                    "institutionName",
                    "institutionCode",
                    "municipalityName",
                ],
            )
        })
        .collect()
}
// ---------------------------------------------------------------------------
// Handlers: writes
// ---------------------------------------------------------------------------

async fn send_message(server: &Server, args: &Value) -> Result<Value, String> {
    let recipient_ids =
        arg_i64_array(args, "recipients").ok_or("recipients (array of ids) is required")?;
    if recipient_ids.is_empty() {
        return Err("recipients must not be empty".to_string());
    }
    let body = arg_str(args, "body").ok_or("body is required")?;
    let subject = arg_str(args, "subject");

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let result = messaging::start_new_thread(
        session,
        &StartNewThreadRequestArguments {
            message: Some(MessageContentRequest {
                attachment_ids: Some(vec![]),
                text: Some(body.clone()),
            }),
            subject: subject.clone(),
            recipients: Some(
                recipient_ids
                    .iter()
                    .map(|&id| RecipientApiModel {
                        id: Some(id),
                        otp_inbox_id: None,
                        mail_box_owner_type: Some("InstitutionProfile".to_string()),
                        profile_id: Some(id),
                        is_deactivated: false,
                        is_deleted: false,
                        portal_role: None,
                    })
                    .collect(),
            ),
            bcc_recipients: Some(vec![]),
            sensitive: false,
            creator: None,
        },
    )
    .await
    .map_err(api_err)?;

    Ok(json!({
        "sent": true,
        "recipients": recipient_ids,
        "subject": subject,
        "body": body,
        "response": result,
    }))
}

async fn reply(server: &Server, args: &Value) -> Result<Value, String> {
    let thread_id = arg_i64(args, "thread_id").ok_or("thread_id is required")?;
    let body = arg_str(args, "body").ok_or("body is required")?;

    let mut guard = server.lock().await?;
    let session = guard.as_mut().expect("session initialised by lock()");
    let result = messaging::reply_to_thread(
        session,
        &ReplyMessageArgument {
            thread_id: Some(thread_id),
            message: Some(MessageContentRequest {
                attachment_ids: Some(vec![]),
                text: Some(body.clone()),
            }),
            common_inbox_id: None,
            bundle_id: None,
        },
    )
    .await
    .map_err(api_err)?;

    Ok(json!({
        "sent": true,
        "thread_id": thread_id,
        "body": body,
        "response": result,
    }))
}
