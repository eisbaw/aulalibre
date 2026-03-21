//! Inode-to-resource mapping.
//!
//! Uses monotonically increasing inode numbers. Never reuses inodes within
//! a mount session. Inode 1 is always the root.

use std::collections::HashMap;
use std::time::SystemTime;

/// Root inode number (FUSE convention).
pub const ROOT_INO: u64 = 1;

/// The type of resource a top-level directory represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Posts,
    Messages,
    Calendar,
    Notifications,
    Gallery,
    Documents,
    Presence,
}

impl ResourceType {
    /// The directory name for this resource type in the filesystem.
    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::Posts => "posts",
            Self::Messages => "messages",
            Self::Calendar => "calendar",
            Self::Notifications => "notifications",
            Self::Gallery => "gallery",
            Self::Documents => "documents",
            Self::Presence => "presence",
        }
    }

    /// All resource types in display order.
    pub fn all() -> &'static [ResourceType] {
        &[
            Self::Posts,
            Self::Messages,
            Self::Calendar,
            Self::Notifications,
            Self::Gallery,
            Self::Documents,
            Self::Presence,
        ]
    }
}

/// What content to produce for a file.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ContentSource {
    /// Plain text content (includes pre-rendered JSON).
    Text(String),
    /// Content to be lazily fetched (e.g., gallery media).
    /// Stores a URL to download from.
    LazyDownload { url: String },
    /// Placeholder content (directory listing).
    Empty,
}

/// An entry in the inode table.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum InodeEntry {
    /// The root directory (inode 1).
    Root,
    /// A top-level resource directory (e.g., `posts/`, `messages/`).
    ResourceDir(ResourceType),
    /// A resource item directory (e.g., `posts/42-Tur til Tivoli/`).
    ResourceItem {
        resource_type: ResourceType,
        name: String,
        created: SystemTime,
        modified: SystemTime,
    },
    /// A file within a resource item.
    File {
        parent_inode: u64,
        name: String,
        content: ContentSource,
        size: u64,
        mtime: SystemTime,
    },
    /// A pagination directory (e.g., `2/` inside `posts/` for page 2).
    PageDir {
        resource_type: ResourceType,
        page: i32,
        parent_inode: u64,
    },
}

/// The inode table mapping inode numbers to entries.
pub struct InodeTable {
    next_ino: u64,
    entries: HashMap<u64, InodeEntry>,
    /// Reverse lookup: parent_inode -> (child_name -> child_inode).
    children: HashMap<u64, HashMap<String, u64>>,
}

impl InodeTable {
    /// Create a new inode table with the root and all resource directories.
    pub fn new() -> Self {
        let mut table = Self {
            next_ino: 2, // 1 is reserved for root
            entries: HashMap::new(),
            children: HashMap::new(),
        };

        // Insert root.
        table.entries.insert(ROOT_INO, InodeEntry::Root);
        table.children.insert(ROOT_INO, HashMap::new());

        // Insert top-level resource directories.
        for rt in ResourceType::all() {
            let ino = table.next_ino;
            table.next_ino += 1;
            table.entries.insert(ino, InodeEntry::ResourceDir(*rt));
            table.children.insert(ino, HashMap::new());
            table
                .children
                .get_mut(&ROOT_INO)
                .unwrap()
                .insert(rt.dir_name().to_string(), ino);
        }

        table
    }

    /// Look up an inode entry.
    pub fn get(&self, ino: u64) -> Option<&InodeEntry> {
        self.entries.get(&ino)
    }

    /// Look up a child inode by parent inode and name.
    pub fn lookup(&self, parent: u64, name: &str) -> Option<u64> {
        self.children.get(&parent)?.get(name).copied()
    }

    /// List children of a directory inode.
    /// Returns (name, inode) pairs.
    pub fn readdir(&self, parent: u64) -> Vec<(String, u64)> {
        self.children
            .get(&parent)
            .map(|kids| kids.iter().map(|(n, i)| (n.clone(), *i)).collect())
            .unwrap_or_default()
    }

    /// Insert a new entry as a child of `parent_ino`.
    /// Returns the newly allocated inode.
    pub fn insert(&mut self, parent_ino: u64, name: String, entry: InodeEntry) -> u64 {
        // Check if this child already exists.
        if let Some(existing) = self.lookup(parent_ino, &name) {
            // Update the existing entry in place.
            self.entries.insert(existing, entry);
            return existing;
        }

        let ino = self.next_ino;
        self.next_ino += 1;

        self.entries.insert(ino, entry);
        self.children.entry(ino).or_default();
        self.children
            .entry(parent_ino)
            .or_default()
            .insert(name, ino);

        ino
    }

    /// Check if a directory has been populated (has any children).
    pub fn has_children(&self, ino: u64) -> bool {
        self.children
            .get(&ino)
            .map(|kids| !kids.is_empty())
            .unwrap_or(false)
    }

    /// Clear all children of a directory (used before re-populating from API).
    #[allow(dead_code)]
    pub fn clear_children(&mut self, parent_ino: u64) {
        if let Some(kids) = self.children.remove(&parent_ino) {
            for child_ino in kids.values() {
                self.remove_recursive(*child_ino);
            }
            self.children.insert(parent_ino, HashMap::new());
        }
    }

    fn remove_recursive(&mut self, ino: u64) {
        if let Some(kids) = self.children.remove(&ino) {
            for child_ino in kids.values() {
                self.remove_recursive(*child_ino);
            }
        }
        self.entries.remove(&ino);
    }

    /// Get the inode for a top-level resource directory.
    #[allow(dead_code)]
    pub fn resource_dir_ino(&self, rt: ResourceType) -> Option<u64> {
        self.lookup(ROOT_INO, rt.dir_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn new_table_has_root_and_dirs() {
        let table = InodeTable::new();
        assert!(table.get(ROOT_INO).is_some());
        assert!(matches!(table.get(ROOT_INO), Some(InodeEntry::Root)));

        // All 7 resource dirs should exist.
        let root_children = table.readdir(ROOT_INO);
        assert_eq!(root_children.len(), 7);
    }

    #[test]
    fn lookup_resource_dirs() {
        let table = InodeTable::new();
        for rt in ResourceType::all() {
            let ino = table.lookup(ROOT_INO, rt.dir_name());
            assert!(ino.is_some(), "missing dir for {:?}", rt);
            assert!(matches!(
                table.get(ino.unwrap()),
                Some(InodeEntry::ResourceDir(_))
            ));
        }
    }

    #[test]
    fn insert_and_lookup() {
        let mut table = InodeTable::new();
        let posts_ino = table.resource_dir_ino(ResourceType::Posts).unwrap();

        let child_ino = table.insert(
            posts_ino,
            "42-Test Post".to_string(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Posts,
                name: "42-Test Post".to_string(),
                created: UNIX_EPOCH,
                modified: UNIX_EPOCH,
            },
        );

        assert_eq!(table.lookup(posts_ino, "42-Test Post"), Some(child_ino));
        assert!(table.has_children(posts_ino));
    }

    #[test]
    fn insert_deduplicates() {
        let mut table = InodeTable::new();
        let posts_ino = table.resource_dir_ino(ResourceType::Posts).unwrap();

        let ino1 = table.insert(
            posts_ino,
            "42-Test".to_string(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Posts,
                name: "42-Test".to_string(),
                created: UNIX_EPOCH,
                modified: UNIX_EPOCH,
            },
        );

        let ino2 = table.insert(
            posts_ino,
            "42-Test".to_string(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Posts,
                name: "42-Test".to_string(),
                created: UNIX_EPOCH,
                modified: UNIX_EPOCH,
            },
        );

        assert_eq!(ino1, ino2);
        assert_eq!(table.readdir(posts_ino).len(), 1);
    }

    #[test]
    fn clear_children() {
        let mut table = InodeTable::new();
        let posts_ino = table.resource_dir_ino(ResourceType::Posts).unwrap();

        table.insert(
            posts_ino,
            "1-A".to_string(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Posts,
                name: "1-A".to_string(),
                created: UNIX_EPOCH,
                modified: UNIX_EPOCH,
            },
        );
        table.insert(
            posts_ino,
            "2-B".to_string(),
            InodeEntry::ResourceItem {
                resource_type: ResourceType::Posts,
                name: "2-B".to_string(),
                created: UNIX_EPOCH,
                modified: UNIX_EPOCH,
            },
        );

        assert_eq!(table.readdir(posts_ino).len(), 2);
        table.clear_children(posts_ino);
        assert_eq!(table.readdir(posts_ino).len(), 0);
    }
}
