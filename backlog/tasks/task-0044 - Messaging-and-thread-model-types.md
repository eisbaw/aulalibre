---
id: TASK-0044
title: Messaging and thread model types
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:08'
updated_date: '2026-03-18 17:32'
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
- [x] #1 MessageThread, Message, MessageRecipient, MailBox structs with serde Deserialize
- [x] #2 Folder, CommonInbox, AutoReply structs
- [x] #3 RichTextWrapperDto for rich text content
- [x] #4 ThreadSubscription and bundle-related types
- [x] #5 Request parameter types for creating/replying to threads
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create aula/aula-api/src/models/messaging.rs with all messaging domain structs
2. Add mod messaging to models/mod.rs
3. Structs organized in sections: shared value types, thread models, subscription models, message models, folder/inbox types, auto-reply, request/argument types
4. Reference enums from crate::enums::messaging and crate::enums::common
5. Reference profile types (ProfilePictureDto, RecipientApiModel uses RecipientApiType)
6. All structs: serde Serialize+Deserialize, camelCase rename, Option for nullable fields
7. Tests for key deserializations
8. Run just e2e
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All structs implemented in messaging.rs:
- Shared types: RichTextWrapperDto, DownloadFileFromAulaArguments, RecipientApiModel, MailBox
- Thread models: MessageThread, SimpleMessageThreadSubscription, ThreadEntityLinkDto, MessageThreadLatestMessage, related child/institution types, MessageParticipantDto, MessageRegardingChildren, MessageDraft
- Subscription: MessageThreadSubscription (full inbox view entity), MessageThreadSubscriptionList
- Messages: MessageDto, MessageRecipient, GetMessageInfoLightDto, MessagesInThreadDto with RecipientDto, MessagesStubbedChild, relation DTOs, MessagingParticipantDto, DeleteMessageDto, UpdateBundleMessageDto, MessageFileUrl
- Folders/Inbox: Folder, CommonInboxesDto
- AutoReply: SetAutoReplyArguments, MessageAutoReplyResult
- Request types: MessageContentRequest, StartNewThreadRequestArguments, ForwardInfoRequestArguments, ForwardThreadRequestArguments, ReplyMessageArgument, EditMessageRequest, AddRecipientArguments, DeleteMessageRequest, DeleteThreadArguments, LeaveThreadArguments, LeaveThreadsRequest, MarkThreadsRequest, MuteThreadRequestArguments, SetLastMessageRequestArguments, SetSensitivityLevelRequest, UpdateMessageThreadsSubscriptionStatusRequest, GetMessageInfoLightRequest, folder CRUD args, GetMessagesForThreadArguments, GetThreadListArguments, GetThreadsInBundleArguments, AttachMessagesToSecureDocumentRequest
- 15 deserialization tests, all passing
- just e2e passes: 193 tests, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added comprehensive messaging domain model types in aula-api/src/models/messaging.rs covering all data_models.md Models.MessageThreads and Models.Messages namespaces.

Changes:
- New file: aula/aula-api/src/models/messaging.rs with 40+ structs spanning the messaging domain
- Registered module in models/mod.rs
- Structs organized by domain: shared value types, thread models, subscription models, message models, folder/inbox, auto-reply, and request/argument types
- All structs derive Serialize + Deserialize with camelCase renaming
- References messaging enums (SensitivityLevel, SubscriptionType, FolderType, ThreadType, CommonInboxType, RecipientApiType) from TASK-42
- References common enums (FilterAndSortType, SortOrderEnum) and profile types (ProfilePictureDto, PortalRole)
- Nullable fields use Option<T>, boolean fields use #[serde(default)]

Tests:
- 15 new deserialization/serialization tests covering key types
- Full e2e suite passes: 193 tests, clippy clean, rustfmt clean
<!-- SECTION:FINAL_SUMMARY:END -->
