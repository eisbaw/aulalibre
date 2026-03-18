---
id: TASK-0053
title: Messaging API service
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:10'
updated_date: '2026-03-18 18:38'
labels:
  - rust
  - aula-api
  - service
dependencies:
  - TASK-0049
  - TASK-0044
references:
  - api_endpoints.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the messaging API service with all 26 methods from api_endpoints.md Section 3.7. Core thread operations: GetThreadList, GetThreadById, StartNewThread, ReplyToThread, DeleteThreads, LeaveThread. Message operations: GetMessageList, DeleteMessage, EditMessage, SetLastReadMessage. Thread management: SetThreadMuted, SetSensitiveLevel, SetThreadMarked, AddRecipientsToThread, ForwardThread, ReplyInNewThread. Auto-reply: SetAutoReply, GetAutoReply, DeleteAutoReply. Folders: GetFolders, CreateFolder, UpdateFolder, DeleteFolder, MoveThreadsToFolder, GetCommonInboxes (from Section 3.13). Bundle/subscription: GetThreadsInBundleList, SetSubscriptionStatus, CheckRecipientsForBlockedChannels, AttachMessagesToSecureDocument.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Thread CRUD: list, get by id, create, reply, delete, leave, forward
- [x] #2 Message operations: list in thread, delete, edit, mark as read
- [x] #3 Thread management: mute, mark, sensitivity level, add recipients
- [x] #4 Auto-reply: set, get, delete
- [x] #5 Folder management: CRUD, move threads, common inboxes
- [x] #6 Bundle/subscription management
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add delete_with_body to AulaClient and Session (DeleteThreads needs DELETE with JSON body)
2. Create messaging.rs service module with all 26+6 methods grouped by AC:
   - Thread CRUD (AC#1): get_thread_list, get_thread_by_id, start_new_thread, reply_to_thread, delete_threads, leave_thread, leave_threads, forward_thread, reply_in_new_thread
   - Message ops (AC#2): get_message_list, delete_message, edit_message, set_last_read_message, get_message_info_light
   - Thread mgmt (AC#3): set_thread_muted, set_thread_marked, set_sensitive_level, add_recipients_to_thread
   - Auto-reply (AC#4): set_auto_reply, get_auto_reply, delete_auto_reply
   - Folders (AC#5): get_folders, create_folder, update_folder, delete_folder, move_threads_to_folder, get_common_inboxes
   - Bundle/subscription (AC#6): get_threads_in_bundle_list, set_subscription_status, check_recipients_for_blocked_channels, attach_messages_to_secure_document, send_event_reminder
3. Register in mod.rs
4. Run just e2e
5. Mark ACs done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented all 32 messaging service methods (26 from MessagingWebService + 6 from FolderService).
Added delete_with_body to AulaClient and Session for DELETE requests with JSON body (needed by DeleteThreads).
All 362 tests pass, clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the complete messaging API service covering all 26 MessagingWebService methods and 6 FolderService methods from the decompiled APK.

Changes:
- Added `delete_with_body` method to `AulaClient` and `Session` for DELETE requests with JSON body (required by DeleteThreads endpoint)
- Created `aula-api/src/services/messaging.rs` with 32 async service functions organized by domain:
  - Thread CRUD: get_thread_list, get_thread_by_id, start_new_thread, reply_to_thread, delete_threads, leave_thread, leave_threads, forward_thread, reply_in_new_thread
  - Message operations: get_message_list, get_message_info_light, delete_message, edit_message, set_last_read_message
  - Thread management: set_thread_muted, set_thread_marked, set_sensitive_level, add_recipients_to_thread
  - Auto-reply: set_auto_reply, get_auto_reply, delete_auto_reply
  - Folders: get_folders, create_folder, update_folder, delete_folder, move_threads_to_folder, get_common_inboxes
  - Bundle/subscription: get_threads_in_bundle_list, set_subscription_status, check_recipients_for_blocked_channels, attach_messages_to_secure_document, send_event_reminder
- Registered module in services/mod.rs
- Added SendEventReminderRequest type and various response type aliases
- 12 unit tests for request serialization

All endpoints use inferred paths from api_endpoints.md Sections 3.7 and 3.13. Query parameter construction handles optional fields for GET endpoints. All existing 362 tests pass.
<!-- SECTION:FINAL_SUMMARY:END -->
