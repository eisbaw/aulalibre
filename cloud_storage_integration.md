# Cloud Storage Integration: Google Drive & OneDrive

## Overview

Aula integrates with Google Drive and OneDrive to allow users to browse, import, and attach cloud-hosted files to messages, posts, and gallery items. The integration operates via a provider abstraction layer with separate OAuth2 flows for each cloud service, fully independent from Aula's primary OIDC login.

This document covers the file operation architecture. For the OAuth authentication flow itself, see `auth_flow.md` Section 11.

---

## 1. Provider Abstraction Layer

### Interface: `CloudStorageIntegrationProviderInterface`

**Namespace:** `AulaNative.ViewModels.CloudStorageIntegration`

The core abstraction is this interface, implemented by both providers:

```csharp
public interface CloudStorageIntegrationProviderInterface
{
    string Name { get; }
    string PackageName { get; }
    string GoToAppButtonName { get; }
    string LinkToApp { get; }
    string LinkToAppStore { get; }

    Task<CloudFolderViewModel> GetRootFolder(CloudStorageOAuthServiceManager mgr);
    Task<CloudFolderViewModel> GetSubFolder(string folderId, string folderName,
                                             CloudStorageOAuthServiceManager mgr, string path = null);
    Task<FileConnectionResult> ImportFile(string fileId, string fileName,
                                          CloudStorageOAuthServiceManager mgr);
}
```

### Implementations

| Class | Namespace | Provider |
|---|---|---|
| `GoogleDriveViewModel` | `AulaNative.ViewModels.CloudStorageIntegration.GoogleDrive` | Google Drive |
| `OneDriveViewModel` | `AulaNative.ViewModels.CloudStorageIntegration.OneDrive` | OneDrive |

### Enum: `CloudStorageService`

```csharp
public enum CloudStorageService { Unknown, GoogleDrive, OneDrive }
```

String mapping (via `CloudStorageServiceEnumHelpers`):
- `"Google Drive"` <-> `CloudStorageService.GoogleDrive`
- `"OneDrive"` <-> `CloudStorageService.OneDrive`

---

## 2. Architecture: Layer-by-Layer

```
UI Layer (Android)
  CloudIntegrationActivity        -- Android Activity for cloud file browsing
  CloudStorageFragment            -- Fragment variant for document section
  CloudIntegrationAdapter         -- RecyclerView adapter (folders + files)

ViewModel Layer (shared .NET)
  CloudStorageIntegrationViewModel  -- Orchestrator: auth check, folder navigation, file import
  GoogleDriveViewModel              -- Google Drive provider implementation
  OneDriveViewModel                 -- OneDrive provider implementation
  CloudFolderViewModel              -- Folder model (FolderId, FolderName, Files, Folders)
  CloudFileViewModel                -- File model (FileId, FileName, FileType, AccessToken)

Service Manager Layer
  CloudStorageOAuthServiceManager   -- Bridges ViewModel <-> HTTP service
    - GetGoogleFilesFromFolder(fileId, fileName)
    - GetGoogleMedia(fileId, fileName)
    - GetOneDriveFilesFromFolder(fileId, fileName, path)
    - GetOneDriveMedia(fileId, fileName)

HTTP Service Layer
  CloudStorageOAuthService          -- Makes actual HTTP calls via CloudService base
    - GetGoogleFilesFromFolder(fileId)  -> GET googleapis.com/drive/v3/files
    - GetOneDriveFilesFromFolder(path)  -> GET graph.microsoft.com/v1.0/me/drive/{path}
  CloudService (base class)
    - CloudGet<T>(url, queryParams, loginData) -- adds Bearer token, executes GET, maps JSON

DTO Layer
  GoogleDriveGetDTO                 -- { NextPageToken, Files: [GoogleDriveGetFileDTO] }
  GoogleDriveGetFileDTO             -- { Id, Name, MimeType, WebViewLink, AccessToken }
  OneDriveGetDTO                    -- { @odata.nextLink, Children: [OneDriveGetChildDTO] }
  OneDriveGetChildDTO               -- { Id, Name, AccessToken, File: OneDriveGetFileDTO }
  OneDriveGetFileDTO                -- { MimeType }

Auth Layer
  CloudStorageAuthenticatorManager  -- OIDC login for cloud providers (2-min token cache)
  CloudStorageAuthenticatorInfoModel -- { ClientId, Scope, AuthUrl, RedirectUrl, AccessTokenUrl }
  AuthenticationState               -- Static in-memory token store (AccountGoogleDrive, AccountOneDrive)
```

---

## 3. OAuth Scopes and Permissions

### Google Drive

| Parameter | Value |
|---|---|
| **Scope** | `https://www.googleapis.com/auth/drive.readonly` |
| **Auth URL** | `https://accounts.google.com/o/oauth2/auth` |
| **Token URL** | `https://oauth2.googleapis.com/token` |
| **Client ID (Android Private Release)** | `811573413698-9bnd25vulk5rt0pfr48hp65rej88a049.apps.googleusercontent.com` |
| **Client ID (Android Private Debug)** | `811573413698-fgk5v7ki9deta3fsr5sama3ervva1o7g.apps.googleusercontent.com` |
| **Client ID (Android Staff Release)** | `839639645203-4bqcf7sudg7j3to5f1coeag1buhb6879.apps.googleusercontent.com` |
| **Client ID (iOS Private)** | `811573413698-h94dnjr2gmaa9pf73fvmdir3732ppg6d.apps.googleusercontent.com` |
| **Client ID (iOS Staff)** | `839639645203-pqk2isies49tmfl34004nq5r5cv4lk2t.apps.googleusercontent.com` |
| **Redirect URL (Android Private)** | `com.netcompany.aulanativeprivate:/googleoauth2redirect` |
| **Secret** | Empty string (public client / PKCE) |
| **API Base URL** | `https://www.googleapis.com/drive/v3/files` |

**Access level:** Read-only. The `drive.readonly` scope grants read access to all files and metadata in the user's Google Drive. The app cannot create, modify, or delete files.

### OneDrive (Microsoft Graph)

| Parameter | Value |
|---|---|
| **Scope** | `https://graph.microsoft.com/files.Read https://graph.microsoft.com/files.Read.all https://graph.microsoft.com/Sites.Read.all` |
| **Auth URL** | `https://login.microsoftonline.com/common/oauth2/v2.0/authorize` |
| **Token URL** | `https://login.microsoftonline.com/common/oauth2/v2.0/token` |
| **Client ID (both Private & Staff)** | `47984900-bb20-4659-9f0d-700f5ab91571` |
| **Redirect URL (Android Private)** | `com.netcompany.aulanativeprivate://onedrive2redirect` |
| **Secret** | Empty string (public client / PKCE) |
| **API Base URL** | `https://graph.microsoft.com/v1.0/me/drive` |

**Access level:** Read-only with broad scope:
- `files.Read` -- user's own files
- `files.Read.all` -- files shared with the user
- `Sites.Read.all` -- SharePoint site content (allows browsing shared drives in organizational contexts)

The `/common` tenant means any Microsoft account (personal or organizational) can authenticate.

### Aula Permission Gates

Cloud storage access is not unconditional. The Aula backend sends per-institution permissions that gate the UI:

| Aula Permission Enum | Enables |
|---|---|
| `ATTACH_GOOGLE_DRIVE_FILE` | Attach Google Drive file links to messages/posts |
| `IMPORT_MEDIA_FROM_GOOGLE_DRIVE` | Download media from Google Drive into Aula gallery |
| `ATTACH_ONEDRIVE_FILE` | Attach OneDrive file links to messages/posts |
| `IMPORT_MEDIA_FROM_ONEDRIVE` | Download media from OneDrive into Aula gallery |

`PermissionManager.GetAllowedCloudProvidersForAttachments()` checks these against the user's institution permissions. If neither Google Drive permission is granted for any institution the user belongs to, the Google Drive option is hidden entirely. Same for OneDrive.

There is also a per-institution variant: `AllowedToUseCloudStorageProvidersForAttachmentsInInstitution(institutionCode)` which checks `ATTACH_GOOGLE_DRIVE_FILE` / `ATTACH_ONEDRIVE_FILE` specifically for a single institution.

---

## 4. End-to-End File Flow

### 4.1 Two Modes of Operation

The `CloudIntegrationActivity` supports two distinct modes, controlled by the `enableDownloadFile` flag:

1. **Link mode** (`enableDownloadFile = false`): User selects a file, and its cloud reference (fileId + accessToken + service name) is attached as a link. The file is NOT downloaded. This is used for attaching cloud files to messages/posts.

2. **Download mode** (`enableDownloadFile = true`): User selects a file, and it is downloaded to a local temp folder (`FilePathUtils.TempCloudPictureFolder`). This is used for importing media into the Aula gallery.

### 4.2 Link Mode Flow (Attaching to Messages/Posts)

```
User taps "Attach from Google Drive" or "Attach from OneDrive"
  |
  v
CloudIntegrationActivity.NewIntent(context, onlyMediaFiles=false,
    enableDownloadFile=false, provider, attachmentAction=callback)
  |
  v
CloudStorageIntegrationViewModel created with provider enum
  |
  v
CloudStorageAuthenticatorManager.GetAccountRequestNewToken(provider)
  -- checks AuthenticationState for cached token (2-min TTL)
  -- if valid token exists: reuse it
  -- if expired or null: trigger OIDC login via OidcClient
  |
  v
[If auth needed] OidcClient.LoginAsync()
  -> Opens WebAuthenticatorBrowser (system browser)
  -> User authenticates with Google/Microsoft
  -> Callback to com.netcompany.aulanativeprivate:{redirect_path}
  -> CloudStorageAuthInterceptor captures the callback
  -> LoginResult with AccessToken returned
  -> CloudStorageAuthenticatorManager.SaveNewAccount()
  -> Saves to AuthenticationState.AccountGoogleDrive/AccountOneDrive
  -> Sets ExpirationDate = DateTime.Now + 2 minutes
  |
  v
CloudStorageIntegrationViewModel.OnAuthCompleted(sender, loginData)
  -> Sets _oAuthSessionAccount on the ViewModel
  -> Creates CloudStorageOAuthServiceManager with the account
  |
  v
CloudStorageIntegrationViewModel.LoadFolder(null)  -- load root
  |
  v
[Google] GoogleDriveViewModel.GetRootFolder(mgr)
  -> calls GetSubFolder("root", "root", mgr)
  -> mgr.GetGoogleFilesFromFolder("root", "root")
  -> CloudStorageOAuthService.GetGoogleFilesFromFolder("root")
  -> GET https://www.googleapis.com/drive/v3/files
       ?pageSize=1000
       &q='root' in parents
       &fields=nextPageToken,files(id,name,mimeType,webViewLink)
       Authorization: Bearer {access_token}
  -> Paginated: follows NextPageToken until exhausted
  -> Each file gets AccessToken injected from the login session
  -> GoogleDriveGetDTO.MapToViewModel() classifies by mimeType:
       application/vnd.google-apps.folder -> Folder
       application/vnd.google-apps.document -> Docs
       application/vnd.google-apps.photo -> Image
       application/vnd.google-apps.spreadsheet -> Sheets
       application/vnd.google-apps.presentation -> Slides
       application/vnd.google-apps.video -> Video
       other -> File (then reclassified by extension)

[OneDrive] OneDriveViewModel.GetRootFolder(mgr)
  -> calls GetSubFolder("root", "root:", mgr)
  -> mgr.GetOneDriveFilesFromFolder("root", "root:", path="root")
  -> CloudStorageOAuthService.GetOneDriveFilesFromFolder("root")
  -> GET https://graph.microsoft.com/v1.0/me/drive/root?$expand=children($select=id,name,webUrl,file)
       Authorization: Bearer {access_token}
  -> Paginated: follows @odata.nextLink until exhausted
  -> Each child gets AccessToken injected
  -> OneDriveGetDTO.MapToViewModel(): File property null -> Folder, else -> File
  |
  v
User browses folders (stack-based navigation in ViewModel)
  -> LoadFolder(subFolder) calls GetSubFolder with folder IDs
  -> GoBackInFolders() pops from _rootFolderStack
  |
  v
User selects a file
  -> LinkFile(cloudFile) is called
  -> _attachmentAction callback invoked with CloudFileViewModel
  -> Activity finishes
  |
  v
Calling code receives CloudFileViewModel:
  -> Creates AulaEditionLinkFileModel(cloudFile, serviceName)
     - Name = cloudFile.FileName
     - ExternalFileId = cloudFile.FileId
     - AccessToken = cloudFile.AccessToken
     - Service = "Google Drive" or "OneDrive"
  -> Added to LinkAttachmentList in AulaEditorFileAttachmentFeature
  |
  v
When message/post is submitted:
  -> Attachments property collects all link files where
     AccessToken != null && ExternalFileId != null
  -> These are sent to the Aula backend as link attachments
  -> Backend stores them as AulaLinkContent: { Service, Name, Url }
```

### 4.3 Download Mode Flow (Gallery Import)

```
User taps "Import from Google Drive" in gallery
  |
  v
CloudIntegrationActivity.NewIntent(context, onlyMediaFiles=true,
    enableDownloadFile=true, provider, galleryAction=callback)
  |
  v
[Same auth + folder browsing flow as link mode]
  |
  v
User selects a file
  -> DownloadFile(cloudFile) is called
  -> CloudStorageIntegrationViewModel.ImportFileFromProvider(cloudFile)
  -> Provider.ImportFile(fileId, fileName, mgr)
  |
  v
[Google] mgr.GetGoogleMedia(fileId, fileName)
  -> URL: https://www.googleapis.com/drive/v3/files/{fileId}?alt=media
  -> Adds Authorization: Bearer header
  -> IFileServiceManager.DownloadFile(url, fileName, TempCloudPictureFolder,
       ignoreError=true, respectNameFromServer=true, httpClient)
  -> Returns FileConnectionResult (local file path)

[OneDrive] mgr.GetOneDriveMedia(fileId, fileName)
  -> URL: https://graph.microsoft.com/v1.0/me/drive/items/{fileId}/content
  -> Same download flow
  -> Returns FileConnectionResult
  |
  v
_galleryAction callback invoked with FileConnectionResult
  -> File is now in TempCloudPictureFolder locally
  -> Gallery handles it as a local media file for upload to Aula
```

### 4.4 File Type Classification

The `CloudStorageIntegrationViewModel` maintains a comprehensive file extension to type mapping:

| Type | Extensions |
|---|---|
| **Docs** | `.doc`, `.dot`, `.wbk`, `.docx`, `.docm`, `.dotx`, `.dotm`, `.docb` |
| **Excel** | `.xls`, `.xlt`, `.xlm`, `.xlsx`, `.xlsm`, `.xltx`, `.xltm`, `.xlsb`, `.xla`, `.xlam`, `.xll`, `.xlw` |
| **PowerPoint** | `.ppt`, `.pot`, `.pps`, `.pptx`, `.pptm`, `.potx`, `.potm`, `.ppam`, `.ppsx`, `.ppsm`, `.sldx`, `.sldm` |
| **Image** | `.bmp`, `.jpeg`, `.jpg`, `.gif`, `.png` |
| **Sound** | `.aac`, `.aif`, `.cda`, `.flac`, `.m4a`, `.mid`, `.midi`, `.mp3`, `.mpa`, `.ogg`, `.wav`, `.wma`, `.wpl` |
| **Video** | `.3g2`, `.3gp`, `.avi`, `.flv`, `.h264`, `.m4v`, `.mkv`, `.mov`, `.mp4`, `.mpg`, `.mpeg`, `.rm`, `.swf`, `.vob`, `.wmv` |
| **PDF** | `.pdf` |

When `OnlyMediaFiles = true`, non-media files (everything except Image, Sound, Video) are filtered out of the listing.

---

## 5. Link Rendering in Messages/Posts

When a message/post contains cloud storage links, special rendering handlers detect and display them:

### `LinkRenderingManager`

Registered handlers match URLs via regex:

| Handler | Regexes | Result |
|---|---|---|
| `GoogleDriveLinkRenderingHandler` | `\\.*drive\\.google\\..*`, `\\.*docs\\.google\\..*` | `DriveLinkRenderingViewModel(url, GoogleDrive)` |
| `OneDriveLinkRenderingHandler` | `\\.*onedrive\\.live\\..*`, `.*1drv\\.ms.*` | `DriveLinkRenderingViewModel(url, OneDrive)` |

The `DriveLinkRenderingViewModel` provides:
- `LinkDescription`: localized "Open in Google Drive" / "Open in OneDrive" text
- `LinkTitle`: the raw URL
- `AccessLinkLabel`: accessibility label
- `iOSInstallAppScheme`: `googledrive://` or `ms-onedrive://` (for deep linking)
- `iOSAppStoreLink`: iTunes link if app not installed

### `AulaLinkContent` (API Model)

Cloud file references stored by the Aula backend use this model:

```csharp
public class AulaLinkContent {
    string Service;            // "Google Drive" or "OneDrive"
    string Name;               // Filename
    string Url;                // Web URL to the file
    CloudStorageService? CloudStorageService;  // Computed from Service string
}
```

---

## 6. File Picker Integration Points

The `AulaFilePickerEnum` flags control which picker options appear in the attachment UI:

| Flag | Value | Context |
|---|---|---|
| `GoogleDrive` | 0x008 | Browse Google Drive for gallery import (media) |
| `OneDrive` | 0x010 | Browse OneDrive for gallery import (media) |
| `DownloadMediaGoogleDrive` | 0x200 | Download media from Google Drive |
| `DownloadMediaOneDrive` | 0x400 | Download media from OneDrive |
| `AttachFileGoogleDrive` | 0x1000 | Attach Google Drive file as link |
| `AttachFileOneDrive` | 0x2000 | Attach OneDrive file as link |

These flags are dynamically set based on the user's institution permissions via `PermissionManager`.

### Where cloud attachments appear:

1. **Message editor** (`AulaEditorFileAttachmentFeature`): Checks `AllowedToUseCloudProvidersForGalleryImport()` and sets both browse flags (GoogleDrive/OneDrive) and attach flags (AttachFileGoogleDrive/AttachFileOneDrive).

2. **Gallery import** (`ImportMediaViewModel`): Checks same permission method but only sets download flags (DownloadMediaGoogleDrive/DownloadMediaOneDrive) and browse flags.

3. **Document overview** (`DocumentOverviewPageViewModel`): Checks `GetAllowedCloudProvidersForAttachments()` and adds Google Drive / OneDrive sections to the document listing page.

---

## 7. Security Observations

1. **Read-only access**: Both providers are configured for read-only scopes. The app cannot modify user's cloud files.

2. **Short-lived token cache**: Cloud storage tokens are cached for only 2 minutes in `AuthenticationState` (in-memory, not persisted). After expiry, a full re-authentication is required.

3. **No token persistence**: Unlike the main Aula session, cloud storage tokens are NOT saved to SecureStorage. They are lost on app restart.

4. **Public client (no secret)**: `CloudConf.secret` is an empty string. The OIDC flow uses PKCE (via IdentityModel.OidcClient) rather than a client secret, which is correct for mobile apps.

5. **Access token passed in file metadata**: Each file DTO receives the access token injected into it (`x.AccessToken = _account.AccessToken`). This is used for the link attachment flow where the token travels with the file reference. This is a potential concern if tokens are logged or persisted unexpectedly, though the 2-minute TTL limits exposure.

6. **Shared OneDrive Client ID**: Both the private and staff Aula apps use the same OneDrive client ID (`47984900-bb20-4659-9f0d-700f5ab91571`). Google Drive uses different client IDs per app variant and platform.

7. **OneDrive scope is broad**: `Sites.Read.all` grants read access to all SharePoint sites the user can access, which in an organizational context could be substantial. This is likely needed for accessing shared/organizational files but is a wider permission than strictly necessary for personal OneDrive access.

8. **Permission gating is client-side**: The `PermissionManager` checks are purely UI-level. The backend controls which permissions a user has, but the cloud OAuth flow itself has no server-side validation - the app talks directly to Google/Microsoft APIs.

---

## 8. Key File Reference

| File | Namespace | Role |
|---|---|---|
| `CloudStorageIntegrationProviderInterface.cs` | `ViewModels.CloudStorageIntegration` | Provider interface |
| `GoogleDriveViewModel.cs` | `ViewModels.CloudStorageIntegration.GoogleDrive` | Google Drive implementation |
| `OneDriveViewModel.cs` | `ViewModels.CloudStorageIntegration.OneDrive` | OneDrive implementation |
| `CloudStorageIntegrationViewModel.cs` | `ViewModels.CloudStorageIntegration` | Orchestrator (auth, nav, import) |
| `CloudFolderViewModel.cs` | `ViewModels.CloudStorageIntegration` | Folder model |
| `CloudFileViewModel.cs` | `ViewModels.CloudStorageIntegration` | File model |
| `CloudStorageOAuthServiceManager.cs` | `ServiceManagers` | Service manager bridge |
| `CloudStorageOAuthService.cs` | `Services.Web` | HTTP API calls |
| `CloudService.cs` | `Services.WebUtils` | Base HTTP + Bearer auth |
| `CloudConf.cs` | `Configuration` | OAuth config constants |
| `CloudStorageAuthenticatorManager.cs` | `OAuth.OAuthCloudStorage` | OIDC login + token cache |
| `CloudStorageAuthenticatorInfoModel.cs` | `OAuth.OAuthCloudStorage` | Auth config model |
| `CloudStorageAuthInterceptor.cs` | `Droid.OAuth` | Android OAuth callback handler |
| `AuthenticationState.cs` | `OAuth` | Static token store |
| `CloudStorageService.cs` | `Enums.CloudStorageIntegration` | Provider enum |
| `CloudStorageServiceEnumHelpers.cs` | `Enums.CloudStorageIntegration` | String<->enum mapping |
| `CloudStorageFileType.cs` | `Enums.CloudStorageIntegration` | File type enum |
| `AulaFilePickerEnum.cs` | `Enums` | File picker flag enum |
| `GoogleDriveGetDTO.cs` | `DTOs.CloudStorage` | Google API response |
| `GoogleDriveGetFileDTO.cs` | `DTOs.CloudStorage` | Google file item |
| `OneDriveGetDTO.cs` | `DTOs.CloudStorage` | OneDrive API response |
| `OneDriveGetChildDTO.cs` | `DTOs.CloudStorage` | OneDrive child item |
| `OneDriveGetFileDTO.cs` | `DTOs.CloudStorage` | OneDrive file metadata |
| `AulaLinkContent.cs` | `Models.Common.Api.Files.Result` | Backend link attachment model |
| `AulaEditionLinkFileModel.cs` | `ViewModels.Files` | Editor link attachment model |
| `AulaLinkFileResultViewModel.cs` | `ViewModels.Files.AulaFileResult` | Link file display model |
| `AulaEditorFileAttachmentFeature.cs` | `CustomViews.RichEditor.File` | Attachment feature (messages/posts) |
| `CloudIntegrationActivity.cs` | `Droid.Activities.Document.CloudIntegration` | Android cloud browser activity |
| `CloudIntegrationAdapter.cs` | `Droid.Activities.Gallery.NewFolder` | RecyclerView adapter |
| `GoogleDriveLinkRenderingHandler.cs` | `Plugins.LinkRenderingManager.Handlers.GoogleDrive` | URL detection |
| `OneDriveLinkRenderingHandler.cs` | `Plugins.LinkRenderingManager.Handlers.OneDrive` | URL detection |
| `DriveLinkRenderingViewModel.cs` | `Plugins.LinkRenderingManager.Handlers` | Link rendering model |
| `PermissionManager.cs` | `Services.Singleton` | Permission checking |
