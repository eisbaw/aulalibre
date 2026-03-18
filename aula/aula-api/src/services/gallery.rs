//! Gallery service.
//!
//! Maps to `AulaNative.Services.Web.GalleryWebService` (12 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_MEDIA` | `gallery.getMedia` |
//! | `GET_MEDIA_WITH_INSTITUTIONID` | `gallery.getMediaByInstitutionProfileId` |
//! | `GET_ALBUM` | `gallery.getAlbums` |
//! | `CREATE_ALBUM` | `gallery.createAlbum` |
//! | `UPDATE_ALBUM` | `gallery.updateAlbum` |
//! | `DELETE_MEDIA` | `gallery.deleteMedia` |
//! | `CREATE_MEDIA` | `gallery.createMedia` |
//! | `UPDATE_MEDIA` | `gallery.updateMedia` |
//! | `MEDIA_ADD_TAG` | `gallery.addTag` |
//! | `MEDIA_REMOVE_TAG` | `gallery.removeTag` |
//! | `REPORT_MEDIA` | `gallery.reportMedia` |
//! | `GET_MEDIA_BY_ID` | `gallery.getMediaById` |
//! | `DELETE_ALBUM` | `gallery.deleteAlbums` |

use crate::models::files::AddOrRemoveTagArguments;
use crate::models::gallery::{
    AlbumDto, CreateAlbumParameters, GalleryViewFilter, GetMediaInAlbumFilter, MediaListDto,
    MediasInAlbumDto,
};
use crate::models::posts::ReportApiParameter;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch albums matching the given filter.
///
/// Maps to `GalleryWebService.GetAlbums()`.
///
/// # Endpoint
///
/// `GET /gallery/albums?<query params>`
pub async fn get_albums(
    session: &mut Session,
    filter: &GalleryViewFilter,
) -> crate::Result<Vec<AlbumDto>> {
    let mut query = Vec::new();
    if let Some(ref code) = filter.selected_institution_code_for_filter {
        query.push(format!("selectedInstitutionCodeForFilter={code}"));
    }
    if let Some(album_id) = filter.album_id {
        query.push(format!("albumId={album_id}"));
    }
    if let Some(user_specific) = filter.user_specific_album {
        query.push(format!("userSpecificAlbum={user_specific}"));
    }
    if let Some(limit) = filter.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(index) = filter.index {
        query.push(format!("index={index}"));
    }
    if let Some(ref sort_on) = filter.sort_on {
        query.push(format!("sortOn={sort_on}"));
    }
    if let Some(ref order) = filter.order_direction {
        query.push(format!("orderDirection={order}"));
    }
    if let Some(ref filter_by) = filter.filter_by {
        query.push(format!("filterBy={filter_by}"));
    }

    let path = if query.is_empty() {
        "?method=gallery.getAlbums".to_string()
    } else {
        format!("?method=gallery.getAlbums&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Fetch albums with caching hint.
///
/// Maps to `GalleryWebService.GetAlbumsCached()`.
///
/// Identical endpoint to `get_albums` but the native app uses a local
/// cache layer. From an API perspective the request is the same; the
/// caching is client-side. This wrapper exists to match the decompiled
/// method one-to-one.
///
/// # Endpoint
///
/// `GET /gallery/albums?<query params>`
pub async fn get_albums_cached(
    session: &mut Session,
    filter: &GalleryViewFilter,
) -> crate::Result<Vec<AlbumDto>> {
    get_albums(session, filter).await
}

/// Fetch media items in a specific album.
///
/// Maps to `GalleryWebService.GetMediasInAlbum()`.
///
/// # Endpoint
///
/// `GET /gallery/albums/{albumId}/media?<query params>`
pub async fn get_medias_in_album(
    session: &mut Session,
    filter: &GetMediaInAlbumFilter,
) -> crate::Result<MediasInAlbumDto> {
    let album_id = filter.album_id.unwrap_or(0);
    let mut query = Vec::new();
    if let Some(user_specific) = filter.user_specific_album {
        query.push(format!("userSpecificAlbum={user_specific}"));
    }
    if let Some(limit) = filter.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(index) = filter.index {
        query.push(format!("index={index}"));
    }
    if let Some(ref sort_on) = filter.sort_on {
        query.push(format!("sortOn={sort_on}"));
    }
    if let Some(ref order) = filter.order_direction {
        query.push(format!("orderDirection={order}"));
    }
    if let Some(ref filter_by) = filter.filter_by {
        query.push(format!("filterBy={filter_by}"));
    }
    if filter.is_selection_mode {
        query.push("isSelectionMode=true".to_string());
    }
    if let Some(ref code) = filter.selected_institution_code {
        query.push(format!("selectedInstitutionCode={code}"));
    }

    let path = if query.is_empty() {
        format!("?method=gallery.getMedia&albumId={album_id}")
    } else {
        format!(
            "?method=gallery.getMedia&albumId={album_id}&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Fetch media items in an album with caching hint.
///
/// Maps to `GalleryWebService.GetMediasInAlbumCached()`.
///
/// Same API call as `get_medias_in_album`; caching is client-side.
///
/// # Endpoint
///
/// `GET /gallery/albums/{albumId}/media?<query params>`
pub async fn get_medias_in_album_cached(
    session: &mut Session,
    filter: &GetMediaInAlbumFilter,
) -> crate::Result<MediasInAlbumDto> {
    get_medias_in_album(session, filter).await
}

/// Fetch a single media item by ID.
///
/// Maps to `GalleryWebService.GetMediaById()`.
///
/// # Endpoint
///
/// `GET /gallery/media/{id}`
pub async fn get_media_by_id(session: &mut Session, media_id: i64) -> crate::Result<MediaListDto> {
    session
        .get(&format!("?method=gallery.getMediaById&id={media_id}"))
        .await
}

/// Create a new album.
///
/// Maps to `GalleryWebService.CreateAlbum()`.
///
/// # Endpoint
///
/// `POST /gallery/albums`
pub async fn create_album(
    session: &mut Session,
    params: &CreateAlbumParameters,
) -> crate::Result<serde_json::Value> {
    session.post("?method=gallery.createAlbum", params).await
}

/// Update an existing album.
///
/// Maps to `GalleryWebService.UpdateAlbum()`.
///
/// # Endpoint
///
/// `PUT /gallery/albums/{id}`
pub async fn update_album(
    session: &mut Session,
    _album_id: i64,
    params: &CreateAlbumParameters,
) -> crate::Result<serde_json::Value> {
    session.post("?method=gallery.updateAlbum", params).await
}

/// Delete an album.
///
/// Maps to `GalleryWebService.DeleteAlbum()`.
///
/// # Endpoint
///
/// `DELETE /gallery/albums/{id}`
pub async fn delete_album(
    session: &mut Session,
    album_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .post(
            "?method=gallery.deleteAlbums",
            &serde_json::json!({"albumId": album_id}),
        )
        .await
}

/// Delete a media item.
///
/// Maps to `GalleryWebService.DeleteMedia()`.
///
/// # Endpoint
///
/// `DELETE /gallery/media/{id}`
pub async fn delete_media(
    session: &mut Session,
    media_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .post(
            "?method=gallery.deleteMedia",
            &serde_json::json!({"mediaId": media_id}),
        )
        .await
}

/// Add a tag (person tag) to a media item.
///
/// Maps to `GalleryWebService.AddTag()`.
///
/// # Endpoint
///
/// `POST /gallery/media/{id}/tags`
pub async fn add_tag(
    session: &mut Session,
    _media_id: i64,
    params: &AddOrRemoveTagArguments,
) -> crate::Result<serde_json::Value> {
    session.post("?method=gallery.addTag", params).await
}

/// Remove a tag from a media item.
///
/// Maps to `GalleryWebService.RemoveTag()`.
///
/// # Endpoint
///
/// `DELETE /gallery/media/{mediaId}/tags/{tagId}`
pub async fn remove_tag(
    session: &mut Session,
    media_id: i64,
    tag_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .post(
            "?method=gallery.removeTag",
            &serde_json::json!({"mediaId": media_id, "tagId": tag_id}),
        )
        .await
}

/// Report a media item for moderation.
///
/// Maps to `GalleryWebService.ReportMedia()`.
///
/// # Endpoint
///
/// `POST /gallery/media/{id}/report`
pub async fn report_media(
    session: &mut Session,
    _media_id: i64,
    params: &ReportApiParameter,
) -> crate::Result<serde_json::Value> {
    session.post("?method=gallery.reportMedia", params).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gallery_view_filter_serializes() {
        let filter = GalleryViewFilter {
            selected_institution_code_for_filter: Some("101001".into()),
            album_id: None,
            user_specific_album: None,
            limit: Some(20),
            index: Some(0),
            sort_on: Some("createdAt".into()),
            order_direction: Some("desc".into()),
            filter_by: None,
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["selectedInstitutionCodeForFilter"], "101001");
        assert_eq!(json["limit"], 20);
        assert_eq!(json["sortOn"], "createdAt");
    }

    #[test]
    fn create_album_params_serialize() {
        let params = CreateAlbumParameters {
            title: Some("Trip Photos".into()),
            album_id: None,
            creator_institution_profile_id: Some(42),
            shared_with_groups: None,
            description: Some("Photos from the trip".into()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["title"], "Trip Photos");
        assert_eq!(json["creatorInstitutionProfileId"], 42);
        assert_eq!(json["description"], "Photos from the trip");
    }

    #[test]
    fn add_or_remove_tag_args_serialize() {
        let args = AddOrRemoveTagArguments {
            inst_profile_id: Some(55),
            media_id: Some(100),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["instProfileId"], 55);
        assert_eq!(json["mediaId"], 100);
    }

    #[test]
    fn get_media_in_album_filter_serializes() {
        let filter = GetMediaInAlbumFilter {
            album_id: Some(10),
            user_specific_album: None,
            limit: Some(50),
            index: Some(0),
            sort_on: None,
            order_direction: None,
            filter_by: None,
            is_selection_mode: false,
            selected_institution_code: None,
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["albumId"], 10);
        assert_eq!(json["limit"], 50);
    }
}
