//! Gallery and media-related enums.

use serde::{Deserialize, Serialize};

/// Media type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaTypeEnum {
    Unknown,
    Image,
    Video,
    Sound,
    MediaWithDuration,
    Media,
}

/// Conversion/processing status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConversionStatusEnum {
    Completed,
    Processing,
    Failed,
}

/// Document change type in gallery context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentChangeType {
    Create,
    Update,
    Delete,
}

/// Dropdown menu actions for gallery items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GalleryDropDownEnumeration {
    Download,
    Delete,
    Report,
    EditTags,
    ViewInfo,
}

/// Image size presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageSizeEnum {
    Original,
    Max200,
    Max400,
}

/// Cell type in media grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaCellType {
    TagCell,
    TaggedCell,
    NonTagCell,
    EmptyCell,
}

/// Batch action on selected media.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaSelectManyAction {
    AddTags,
    Download,
    Delete,
    EditInfo,
    RotateRight,
}

/// Rotation angle for images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RotatingEnum {
    Rotating0,
    Rotating90,
    Rotating180,
    Rotating270,
}

/// Thumbnail size presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThumbnailSizeEnum {
    XS,
    S,
    M,
    L,
    Full,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! roundtrip_test {
        ($name:ident, $ty:ty, $variant:expr) => {
            #[test]
            fn $name() {
                let json = serde_json::to_string(&$variant).unwrap();
                let back: $ty = serde_json::from_str(&json).unwrap();
                assert_eq!(back, $variant);
            }
        };
    }

    roundtrip_test!(media_type, MediaTypeEnum, MediaTypeEnum::Video);
    roundtrip_test!(
        conversion_status,
        ConversionStatusEnum,
        ConversionStatusEnum::Processing
    );
    roundtrip_test!(doc_change, DocumentChangeType, DocumentChangeType::Update);
    roundtrip_test!(
        gallery_dropdown,
        GalleryDropDownEnumeration,
        GalleryDropDownEnumeration::EditTags
    );
    roundtrip_test!(image_size, ImageSizeEnum, ImageSizeEnum::Max400);
    roundtrip_test!(media_cell, MediaCellType, MediaCellType::TaggedCell);
    roundtrip_test!(
        media_select,
        MediaSelectManyAction,
        MediaSelectManyAction::RotateRight
    );
    roundtrip_test!(rotating, RotatingEnum, RotatingEnum::Rotating270);
    roundtrip_test!(thumbnail_size, ThumbnailSizeEnum, ThumbnailSizeEnum::XS);
}
