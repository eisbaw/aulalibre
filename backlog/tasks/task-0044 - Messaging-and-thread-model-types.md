---
id: TASK-0044
title: Messaging and thread model types
status: To Do
assignee: []
created_date: '2026-03-18 16:08'
labels:
  - rust
  - aula-api
  - models
dependencies:
  - TASK-0042
references:
  - data_models.md
  - domain_concepts.md
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Rust structs for the messaging domain: MessageThread, Message, MessageDto, MessageRecipient, MailBox, Folder, CommonInbox, AutoReply, RichTextWrapperDto, ThreadSubscription. Threads support sensitivity levels (Level1-3 where Level3 requires MitID step-up), folders (Normal, Deleted), muting, marking, bundling (SubscriptionType), BCC recipients, and regarding-children tagging. See data_models.md Models.MessageThreads and Models.Messages namespaces, and domain_concepts.md Section 1.3.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 MessageThread, Message, MessageRecipient, MailBox structs with serde Deserialize
- [ ] #2 Folder, CommonInbox, AutoReply structs
- [ ] #3 RichTextWrapperDto for rich text content
- [ ] #4 ThreadSubscription and bundle-related types
- [ ] #5 Request parameter types for creating/replying to threads
<!-- AC:END -->
