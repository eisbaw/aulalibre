//! Gallery and album models.
//!
//! Types from `Models.Gallery` covering albums, media lists,
//! album creation/deletion, and gallery filters.
//!
//! See `data_models.md` Models.Gallery namespace.

use serde::{Deserialize, Serialize};

use super::files::{
    AulaFileResultDto, AulaGalleryMediaFileResultDto, LinkedGroupRequestModel, ShareWithGroupDto,
};
use super::messaging::DownloadFileFromAulaArguments;

// ---------------------------------------------------------------------------
// Album and media DTOs
// ---------------------------------------------------------------------------

/// Album creator profile.
///
/// Maps to `Models.Gallery.GalleryDto.AlbumCreatorDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCreatorDto {
    pub id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub profile_id: Option<i64>,
    pub role: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

/// Group context in album views.
///
/// Maps to `Models.Gallery.GalleryDto.AlbumGroupDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumGroupDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub role: Option<String>,
    pub main_group: Option<bool>,
}

/// Full album with metadata, thumbnails, and permissions.
///
/// Maps to `Models.Gallery.AlbumDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDto {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub creator: Option<AlbumCreatorDto>,
    pub creation_date: Option<String>,
    pub total_size: Option<i32>,
    pub size: Option<i32>,
    pub from: Option<i32>,
    pub description: Option<String>,
    pub shared_with_groups: Option<Vec<ShareWithGroupDto>>,
    pub thumbnails_urls: Option<Vec<String>>,
    #[serde(default)]
    pub current_user_can_edit: bool,
    #[serde(default)]
    pub current_user_can_delete: bool,
    #[serde(default)]
    pub current_user_can_add_media: bool,
}

/// Media creator profile (used in gallery listings).
///
/// Maps to `Models.Gallery.MediaCreatorModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCreatorModel {
    pub id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub role: Option<String>,
    pub profile_id: Option<i64>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
}

/// List of media items (used for gallery views without album context).
///
/// Maps to `Models.Gallery.MediaListDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListDto {
    pub results: Option<Vec<AulaFileResultDto>>,
    pub album: Option<AlbumDto>,
}

/// Media items within an album (with comment counts).
///
/// Maps to `Models.Gallery.MediasInAlbumDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediasInAlbumDto {
    pub results: Option<Vec<AulaGalleryMediaFileResultDto>>,
    pub album: Option<AlbumDto>,
    pub media_count: Option<i32>,
}

// ---------------------------------------------------------------------------
// Gallery parameters
// ---------------------------------------------------------------------------

/// Parameters for creating an album.
///
/// Maps to `Models.Gallery.GalleryParameters.CreateAlbumParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbumParameters {
    pub title: Option<String>,
    pub album_id: Option<i64>,
    pub creator_institution_profile_id: Option<i64>,
    pub shared_with_groups: Option<Vec<LinkedGroupRequestModel>>,
    pub description: Option<String>,
}

/// Parameters for deleting albums.
///
/// Maps to `Models.Gallery.GalleryParameters.DeleteAlbumParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAlbumParameters {
    pub album_ids: Option<Vec<i64>>,
}

/// Filter/sort for gallery view.
///
/// Maps to `Models.Gallery.GalleryParameters.GalleryViewFilter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryViewFilter {
    pub selected_institution_code_for_filter: Option<String>,
    pub album_id: Option<i64>,
    pub user_specific_album: Option<bool>,
    pub limit: Option<i32>,
    pub index: Option<i32>,
    pub sort_on: Option<String>,
    pub order_direction: Option<String>,
    pub filter_by: Option<String>,
}

/// Filter/sort for media-in-album view.
///
/// Maps to `Models.Gallery.GalleryParameters.GetMediaInAlbumFilter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaInAlbumFilter {
    pub album_id: Option<i64>,
    pub user_specific_album: Option<bool>,
    pub limit: Option<i32>,
    pub index: Option<i32>,
    pub sort_on: Option<String>,
    pub order_direction: Option<String>,
    pub filter_by: Option<String>,
    #[serde(default)]
    pub is_selection_mode: bool,
    pub selected_institution_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_album_dto() {
        let json = r#"{
            "id": 10,
            "title": "Tur til Tivoli",
            "name": "Tur til Tivoli",
            "creator": {
                "id": 55,
                "institutionCode": "101001",
                "institutionName": "Test Skole",
                "name": "Lars Hansen",
                "shortName": "LH",
                "metadata": null,
                "profileId": 33,
                "role": "Employee",
                "profilePicture": null
            },
            "creationDate": "2024-03-15T08:00:00",
            "totalSize": 42,
            "size": 20,
            "from": 0,
            "description": "Photos from the trip",
            "sharedWithGroups": [],
            "thumbnailsUrls": ["https://cdn.aula.dk/thumb1.jpg"],
            "currentUserCanEdit": true,
            "currentUserCanDelete": false,
            "currentUserCanAddMedia": true
        }"#;
        let a: AlbumDto = serde_json::from_str(json).unwrap();
        assert_eq!(a.id, Some(10));
        assert_eq!(a.title.as_deref(), Some("Tur til Tivoli"));
        assert!(a.current_user_can_edit);
        assert!(!a.current_user_can_delete);
        assert_eq!(a.total_size, Some(42));
        let creator = a.creator.unwrap();
        assert_eq!(creator.name.as_deref(), Some("Lars Hansen"));
    }

    #[test]
    fn deserialize_media_list_dto() {
        let json = r#"{
            "results": [],
            "album": null
        }"#;
        let m: MediaListDto = serde_json::from_str(json).unwrap();
        assert!(m.album.is_none());
        assert_eq!(m.results.unwrap().len(), 0);
    }

    #[test]
    fn deserialize_gallery_view_filter() {
        let json = r#"{
            "selectedInstitutionCodeForFilter": "101001",
            "albumId": null,
            "userSpecificAlbum": null,
            "limit": 20,
            "index": 0,
            "sortOn": "createdAt",
            "orderDirection": "desc",
            "filterBy": null
        }"#;
        let f: GalleryViewFilter = serde_json::from_str(json).unwrap();
        assert_eq!(f.limit, Some(20));
        assert_eq!(f.sort_on.as_deref(), Some("createdAt"));
    }

    #[test]
    fn deserialize_create_album_parameters() {
        let json = r#"{
            "title": "New Album",
            "albumId": null,
            "creatorInstitutionProfileId": 42,
            "sharedWithGroups": [{"groupId": 5, "portalRolesEnum": ["Guardian"]}],
            "description": "Test album"
        }"#;
        let p: CreateAlbumParameters = serde_json::from_str(json).unwrap();
        assert_eq!(p.title.as_deref(), Some("New Album"));
        assert_eq!(p.creator_institution_profile_id, Some(42));
    }

    #[test]
    fn deserialize_album_group_dto() {
        let json = r#"{
            "id": 5,
            "name": "3.A",
            "institutionCode": "101001",
            "institutionName": "Test Skole",
            "role": "Employee",
            "mainGroup": true
        }"#;
        let g: AlbumGroupDto = serde_json::from_str(json).unwrap();
        assert_eq!(g.id, Some(5));
        assert_eq!(g.main_group, Some(true));
    }
}
