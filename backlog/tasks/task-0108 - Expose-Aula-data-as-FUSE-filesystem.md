---
id: TASK-0108
title: Expose Aula data as FUSE filesystem
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-20 22:26'
updated_date: '2026-03-21 00:25'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Mount Aula data as a FUSE filesystem where each CLI command/resource becomes a folder, preserving the natural hierarchy (e.g. institutions/posts, threads/messages). Inodes should carry sensible creation/modification times derived from the API data. File and folder names may need sanitization (illegal chars) or shortening (length limits).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 FUSE mount exposes Aula resources as a directory tree preserving API hierarchy
- [x] #2 Each resource type (posts, messages, calendar, etc.) maps to a folder
- [x] #3 Inode timestamps (ctime/mtime) reflect actual creation/modification times from API data
- [x] #4 File/folder names are sanitized for filesystem safety (no illegal chars, length limits)
- [x] #5 Read operations work without errors for browsing and reading content
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan for FUSE Filesystem

### 1. FUSE Crate Selection: `fuser`

Use `fuser` (https://crates.io/crates/fuser) -- the actively maintained successor to the `fuse-rs` crate. Reasons:
- Pure Rust, no C libfuse dependency (uses kernel interface directly on Linux)
- Well-documented trait-based API (`Filesystem` trait)
- Async-friendly (though the trait itself is sync, we can use `tokio::runtime::Handle::block_on` for API calls)
- Available in nixpkgs/crates.io, compatible with our existing toolchain
- Requires `libfuse3-dev` (or `fuse3` in nix) as a build dependency for the FUSE kernel headers

Alternative considered: `polyfuse` -- more async-native but less mature and less documented. Not worth the risk for a first implementation.

### 2. New Crate: `aula-fuse`

Add a third workspace member: `aula/aula-fuse/`. This keeps the FUSE filesystem concern separate from both the API library and the CLI.

```
aula/
  Cargo.toml          # workspace: add "aula-fuse"
  aula-api/            # existing library crate
  aula-cli/            # existing CLI binary
  aula-fuse/           # NEW: FUSE filesystem binary
    Cargo.toml
    src/
      main.rs          # arg parsing, mount, signal handling
      fs.rs            # Filesystem trait implementation
      inode_table.rs   # inode <-> path/resource mapping
      cache.rs         # TTL-based API response cache
      sanitize.rs      # filename sanitization
      timestamp.rs     # Aula datetime string -> SystemTime conversion
```

Dependencies for aula-fuse:
- `aula-api` (path dep) -- reuse Session, services, models
- `fuser` -- FUSE implementation
- `clap` -- mount point arg, env/verbose flags
- `tokio` -- async runtime for API calls
- `chrono` -- parse Aula datetime strings
- `log` + `env_logger` -- structured logging

### 3. Directory Hierarchy Design

The filesystem mirrors the CLI command structure and API resource hierarchy. All content is read-only.

```
<mountpoint>/
  posts/
    <id>-<sanitized_title>/
      content.txt          # HTML body stripped to text (or content.html for raw)
      metadata.json        # full PostApiDto as JSON
  messages/
    <thread_id>-<sanitized_subject>/
      metadata.json        # MessageThreadSubscription as JSON
      <msg_id>-<sender_short>.txt   # message body per message
  calendar/
    <event_id>-<sanitized_title>/
      details.txt          # event description, times, location
      metadata.json        # full CalendarEventDto as JSON
  notifications/
    <notification_id>.json       # NotificationItemDto as JSON (flat list)
  gallery/
    <album_id>-<sanitized_title>/
      metadata.json        # AlbumDto as JSON
      # media items listed but NOT auto-downloaded (too expensive)
  documents/
    <doc_id>-<sanitized_title>/
      content.txt          # document body (if internal)
      metadata.json        # SecureDocumentDto as JSON
  presence/
    <child_name>/
      status.txt           # current presence status
      metadata.json        # PresenceRegistrationResult as JSON
```

Key design decisions:
- Top-level folders are static and correspond to CLI domains
- Resource folders use `<id>-<sanitized_title>` naming for uniqueness + readability
- `metadata.json` files provide the full API object for programmatic consumers
- `.txt` files provide human-readable extracted content
- Gallery media files are NOT downloaded automatically -- only metadata is shown. Downloading could be a follow-up task or triggered by reading a specific file.
- Thread messages are lazily loaded: readdir on a thread folder triggers the messages-in-thread API call

### 4. Inode Design and Mapping

Use a simple inode table (HashMap<u64, InodeEntry>) that maps inode numbers to resource identifiers.

```rust
enum InodeEntry {
    Root,                                    // inode 1
    ResourceDir(ResourceType),               // e.g. inode 2 = posts/
    ResourceItem {
        resource_type: ResourceType,
        id: ResourceId,                      // API object ID
        created: Option<SystemTime>,         // from API timestamp
        modified: Option<SystemTime>,        // from API timestamp
    },
    File {
        parent_inode: u64,
        name: String,
        content_source: ContentSource,       // how to produce file content
        size: Option<u64>,                   // cached after first read
        mtime: Option<SystemTime>,
    },
}
```

Inode allocation: monotonically increasing counter. Never reuse inodes within a mount session. This is simpler and avoids subtle bugs compared to hash-based inode assignment.

### 5. Timestamp Mapping

Aula API timestamps are ISO 8601-ish strings like `"2024-03-15T08:00:00"` (no timezone, implicitly Europe/Copenhagen).

Strategy:
- Parse with chrono::NaiveDateTime::parse_from_str using format `%Y-%m-%dT%H:%M:%S`
- Assume Europe/Copenhagen timezone (Aula is Denmark-only)
- Convert to SystemTime via chrono's timezone handling
- Fallback: if parsing fails, use UNIX_EPOCH (making it obvious something is wrong rather than silently using current time)

Mapping to inode times:
- `mtime` = most recent modification timestamp from API (e.g. `editedAt` for posts, `sendDateTime` for messages, `updatedAt` for documents)
- `ctime` = creation timestamp (e.g. `timeStamp` for posts, `startedDateTime` for threads, `createdAt` for documents)
- `atime` = same as mtime (no meaningful "access time" concept in API data)

Per-resource timestamp fields:
- Posts: ctime=timeStamp, mtime=editedAt || timeStamp
- Messages: ctime=sendDateTime, mtime=sendDateTime
- Calendar events: ctime=startDateTime, mtime=startDateTime (or lastEditDateTime if available)
- Notifications: ctime=triggered, mtime=triggered
- Gallery albums: ctime=creationDate, mtime=creationDate
- Documents: ctime=createdAt, mtime=updatedAt || createdAt
- Presence: ctime=now (ephemeral data), mtime=now

### 6. Filename Sanitization

Requirements:
- Filesystem-safe: no `/`, `\0`
- Practical: no control chars, no chars that cause shell quoting issues
- Length: max 200 bytes (well under the 255-byte ext4/FUSE limit, leaving room for prefix)
- Human-readable: preserve as much of the original as practical

Strategy (implemented in sanitize.rs):
1. Replace `/` with `-`
2. Replace `\0` with empty
3. Replace control characters (0x00-0x1F, 0x7F) with empty
4. Replace `<>:"|?*\\` with `_` (Windows-compat, also avoids shell pain)
5. Collapse consecutive whitespace to single space
6. Trim leading/trailing whitespace and dots
7. If result is empty after sanitization, use `_unnamed`
8. Truncate to 200 bytes on a UTF-8 char boundary
9. Prefix with `<id>-` to guarantee uniqueness within a directory

This is a pure function with no state -- easy to test exhaustively.

### 7. Caching Strategy

The FUSE filesystem must not hammer the Aula API on every `readdir`/`getattr`/`read`. Strategy: TTL-based in-memory cache per resource type.

```rust
struct Cache {
    entries: HashMap<CacheKey, CacheEntry>,
}

struct CacheEntry {
    data: CachedData,       // serialized API response
    fetched_at: Instant,
    ttl: Duration,
}
```

TTL values (configurable via CLI flags):
- Resource lists (readdir): 5 minutes (posts list, thread list, etc.)
- Individual resources (read): 10 minutes (post detail, thread messages)
- Presence: 2 minutes (more dynamic)

Cache invalidation: only time-based. No push/websocket integration. User can remount to force refresh, or we could expose a `.control` file that accepts commands (follow-up task).

The cache stores deserialized API objects, not raw bytes. File content (the .txt/.json renderings) is computed on demand from cached API objects. This avoids duplicating state -- the API response is the single source of truth, and file content is a derived view.

### 8. Session and Threading Concerns

`fuser` calls `Filesystem` trait methods from multiple threads. The `Session` type from `aula-api` is `&mut self` based (not thread-safe as-is). Solutions:

- Wrap `Session` in `Arc<Mutex<Session>>`
- Each FUSE callback acquires the mutex, uses `tokio::runtime::Handle::current().block_on()` to run the async API call
- This serializes API calls, which is fine -- we do not need concurrent API requests for a single-user filesystem
- The cache sits outside the session mutex (or inside a separate RwLock) so getattr can check cache without blocking on API calls

### 9. Error Handling

- API errors -> return FUSE EIO with eprintln log message
- Network timeout -> return FUSE ETIMEDOUT
- Not logged in -> refuse to mount with clear error message
- Token refresh failure mid-session -> return EIO, log the error, user needs to re-auth and remount
- Invalid inode lookups -> ENOENT
- Write operations (create, write, unlink, etc.) -> EROFS (read-only filesystem)

### 10. Build and Shell Integration

- Add `fuse3` to shell.nix buildInputs (FUSE3 kernel headers and libfuse3)
- Add a `just mount` recipe: `cargo run -p aula-fuse -- /tmp/aula`
- Add a `just umount` recipe: `fusermount3 -u /tmp/aula`
- Add `fuser` to Cargo.toml of aula-fuse

### 11. Risks and Limitations

- **Timestamps without timezone**: Aula timestamps lack timezone info. Assuming Europe/Copenhagen is correct for the production API but could be wrong for test environments. Mitigation: make timezone configurable.
- **Large datasets**: If a user has hundreds of message threads, the initial readdir will be slow. Mitigation: paginate lazily, show only first N items initially. Could add pagination via directory naming (e.g., `page-1/`, `page-2/`) but that adds complexity -- defer to follow-up.
- **Token expiry during long mount**: Session token refresh should handle this, but if the refresh token itself expires (e.g., after days of mounting), the user must re-authenticate. The filesystem should gracefully return errors rather than panic.
- **No write support**: This is read-only. Writing (e.g., creating posts by writing files) would be a significant follow-up.
- **Gallery media**: Not auto-downloading media. Only metadata. Downloading would require significant bandwidth and storage.
- **FUSE performance**: fuser's sync trait means API calls block the FUSE thread. With the mutex approach, only one API call runs at a time. For a single-user filesystem this is acceptable.
- **Stale data**: The cache TTL means data can be up to 5-10 minutes stale. No mechanism for push updates.
- **Thread safety of Session**: The Mutex<Session> approach serializes all API calls. If this becomes a bottleneck (unlikely for single-user), we could switch to a channel-based design with a dedicated API worker thread.

### 12. Implementation Order

1. Create `aula-fuse` crate skeleton with main.rs (clap args: mountpoint, --env, --verbose)
2. Implement `sanitize.rs` with tests
3. Implement `timestamp.rs` with tests
4. Implement `inode_table.rs` -- static directory tree (no API calls yet)
5. Implement `fs.rs` -- Filesystem trait with getattr/readdir for static dirs
6. Test mounting with static dirs (no API)
7. Implement `cache.rs`
8. Wire up posts: readdir lists posts, read returns content
9. Wire up messages, calendar, notifications, documents, gallery, presence
10. Add proper error handling and logging throughout
11. Update shell.nix with fuse3 dependency
12. Add just recipes for mount/umount

### 13. Acceptance Criteria Mapping

- AC#1 (directory tree preserving hierarchy): Steps 4-6, then 8-9
- AC#2 (resource type folders): Steps 4-5
- AC#3 (inode timestamps): Step 3, wired in step 8-9
- AC#4 (filename sanitization): Step 2
- AC#5 (read operations work): Steps 8-9, tested manually
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Design decisions confirmed:
- Gallery media: lazy download on read() (on-demand)
- Pagination: nested folder structure (page 1/ contains next page 2/ as subfolder, and so on)
- Timestamps: Europe/Copenhagen assumption is fine, no configurability needed

Starting implementation. Codebase reviewed: Session wraps AulaClient + TokenStore, services take &mut Session, models use serde with camelCase rename.

Implementation complete. All modules created and wired:

- sanitize.rs: 12 unit tests, handles Danish chars, truncation, special chars
- timestamp.rs: 9 unit tests, CET/CEST timezone handling, DST ambiguity
- inode_table.rs: 5 unit tests, monotonic inode allocation with dedup
- cache.rs: TTL-based cache (5min lists, 10min items, 2min presence)
- fs.rs: Full Filesystem trait impl with 7 resource types, lazy population, nested pagination dirs, lazy gallery download, HTML stripping
- main.rs: Clap CLI with --env and --verbose flags, token verification

Build: clean compile, 32 unit tests pass, clippy -D warnings clean, cargo fmt clean.
shell.nix: fuse3 added to buildInputs.
Justfile: mount/umount recipes added.

Key design decisions implemented:
- Gallery media: lazy download on read() via LazyDownload content source
- Pagination: nested folder structure (page N dir inside parent)
- Timestamps: Europe/Copenhagen assumption via chrono-tz, UNIX_EPOCH fallback
- Thread safety: Arc<Mutex<Session>> with tokio Handle::block_on for async calls
- Error handling: EIO for API errors, ENOENT for missing inodes, EROFS for write ops
<!-- SECTION:NOTES:END -->
