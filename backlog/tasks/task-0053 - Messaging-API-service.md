---
id: TASK-0053
title: Messaging API service
status: To Do
assignee: []
created_date: '2026-03-18 16:10'
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
- [ ] #1 Thread CRUD: list, get by id, create, reply, delete, leave, forward
- [ ] #2 Message operations: list in thread, delete, edit, mark as read
- [ ] #3 Thread management: mute, mark, sensitivity level, add recipients
- [ ] #4 Auto-reply: set, get, delete
- [ ] #5 Folder management: CRUD, move threads, common inboxes
- [ ] #6 Bundle/subscription management
<!-- AC:END -->
