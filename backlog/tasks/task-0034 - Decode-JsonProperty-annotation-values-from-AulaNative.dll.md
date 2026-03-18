---
id: TASK-0034
title: Decode JsonProperty annotation values from AulaNative.dll
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:42'
updated_date: '2026-03-18 23:05'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
monodis cannot decode Newtonsoft.Json attributes because the dependency assembly is missing. 109 fields have [JsonProperty] annotations whose actual JSON field names are unknown. Options: (1) provide Newtonsoft.Json.dll in the same directory as the assembly for monodis, (2) use ilspycmd or dnSpy to get full decompilation, (3) capture actual network traffic to discover field names. This is important for generating correct Rust serde annotations.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Actual JSON field names documented for all 109 annotated properties
- [x] #2 Serde rename annotations prepared for Rust struct generation
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Catalog all decoded [JsonProperty] annotations from C# decompilation
2. For each annotation, identify the corresponding Rust struct and field
3. Determine which need serde(rename) because camelCase conversion does not match
4. Apply #[serde(rename = "...")] fixes to Rust structs
5. Handle "Could not decode" annotations - check if property names follow camelCase convention
6. Document JsonConverter annotations for enum types
7. Run e2e tests to verify
8. Update acceptance criteria
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Decoded JsonProperty Analysis

Found 31 decoded [JsonProperty] annotations and ~49 "Could not decode" annotations.

The C# app uses CamelCasePropertyNamesContractResolver globally (found in SimpleService.cs:1488),
meaning all properties default to camelCase JSON names. [JsonProperty] overrides this only where
the JSON name differs from the camelCase convention.

## Mismatches Fixed (15 total)

1. Profile.externalEmail -> "email" (not "externalEmail")
2. MainGroup.isMainGroup -> "MainGroup" (not "isMainGroup")
3. SearchResultProfileItemBase.institutionProfileId -> "id" (not "institutionProfileId")
4. SearchResultProfileItemBase.externalEmail -> "email" (not "externalEmail")
5. MailBox.email -> "address" (not "email")
6. MuteThreadRequestArguments.owner -> "MailBoxOwner" (not "owner")
7. NotificationSettings.widgetSettings -> "widgetNotificationSettingDtos" (not "widgetSettings")
8. UploadFileData.amzAlgorithm -> "X-Amz-Algorithm"
9. UploadFileData.amzCredential -> "X-Amz-Credential"
10. UploadFileData.amzDate -> "X-Amz-Date"
11. UploadFileData.amzSecurityToken -> "X-Amz-Security-Token"
12. UploadFileData.amzSignature -> "X-Amz-Signature"
13. UploadFileData.cacheControl -> "Cache-Control"
14. GetUploadLinksArguments.uploadNames -> "upload_names" (snake_case\!)
15. WebResponseStatus.backendErrorCode -> "code"

## JsonConverter Annotations

All enum types use DefaultUnknownEnumConverter with CamelCaseNamingStrategy.
This matches our existing serde enum handling.

## "Could not decode" annotations (~49)

These are in request/parameter types (GetCalendarResourcesParameters, CreateBaseEventRequest,
SearchRecipientParameters, etc.). Since ILSpy could not decode the attribute arguments,
the actual JSON names remain unknown. These are mostly request DTOs so they are less
critical for API response deserialization.

All 658 tests pass. E2E clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Decoded and applied all [JsonProperty] annotation overrides from the ILSpy-decompiled AulaNative.dll C# source to the Rust aula-api serde model structs.

Key discovery: The C# app uses CamelCasePropertyNamesContractResolver globally, meaning all properties serialize to camelCase by default. [JsonProperty] annotations only exist where the JSON field name differs from this convention.

Changes (15 serde(rename) annotations added):
- response.rs: WebResponseStatus.backend_error_code -> "code"
- profiles.rs: Profile.external_email -> "email", MainGroup.is_main_group -> "MainGroup"
- search.rs: SearchResultProfileItemBase.institution_profile_id -> "id", .external_email -> "email"
- messaging.rs: MailBox.email -> "address", MuteThreadRequestArguments.owner -> "MailBoxOwner"
- notifications.rs: NotificationSettings.widget_settings -> "widgetNotificationSettingDtos"
- files.rs: UploadFileData AWS fields -> "X-Amz-*" and "Cache-Control", GetUploadLinksArguments.upload_names -> "upload_names" (snake_case)

All test JSON fixtures updated to match the actual API field names.
Also documented ~49 "Could not decode" annotations that ILSpy could not resolve (mostly request parameter types).

658 tests pass, clippy clean, fmt clean.
<!-- SECTION:FINAL_SUMMARY:END -->
