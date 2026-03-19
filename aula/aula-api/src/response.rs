//! API response wrapper types.
//!
//! The Aula API wraps all responses in an envelope pattern typical of .NET APIs.
//! These types mirror the `Models.Web` namespace from the decompiled APK
//! (see `data_models.md` Section "Models.Web").

use serde::Deserialize;

/// Response status block returned by the API.
///
/// Maps to `WebResponseStatus` from the decompiled assembly.
/// Present in most API responses as the `status` field.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebResponseStatus {
    /// HTTP status code echoed by the API (not always present).
    #[serde(default)]
    pub http_code: i32,

    /// Backend-specific error code (0 typically means success).
    /// C# `WebResponseStatus.BackendErrorCode` has `[JsonProperty("code")]`.
    #[serde(default, rename = "code")]
    pub backend_error_code: i32,

    /// Machine-readable error message.
    #[serde(default)]
    pub message: Option<String>,

    /// Human-readable error message for display.
    #[serde(default)]
    pub presented_message: Option<String>,

    /// Sub-code for finer error classification.
    /// See [`WebResponseStatusSubCode`] for known values.
    #[serde(default)]
    pub sub_code: Option<i32>,

    /// HTML content to display on error (e.g., maintenance pages).
    #[serde(default)]
    pub html_content_if_error: Option<String>,
}

/// Known sub-code constants from `WebResponseStatusSubCodeConstants`.
///
/// These are integer constants in the original .NET code. We represent them
/// as an enum with explicit discriminants so they can be matched against
/// the `sub_code` field in [`WebResponseStatus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WebResponseStatusSubCode {
    AuthorizationDeniedAnyScope = 1,
    AuthorizationDeniedInstitutionScope = 2,
    AuthorizationDeniedGroupScope = 3,
    AuthorizationDeniedProfileScope = 4,
    AuthorizationDeniedBlockedCommunication = 5,
    AuthorizationDeniedAccessNotGranted = 6,
    AuthorizationDeniedUserDeactivated = 7,
    AuthorizationStepUpRequired = 8,
    InvalidToken = 9,
    OutOfSyncPresenceConfiguration = 10,
    UnregisterDeviceFailed = 11,
    CrossMunicipalityTagging = 12,
    SessionExpired = 13,
    ExceedingMaximumParticipants = 14,
    DateAlreadyHasOccurrenceFromSameSeries = 15,
    FirstRepeatingEventExceptionOutOfRange = 16,
    LastRepeatingEventExceptionOutOfRange = 17,
    DeactivatedInstitutionProfile = 18,
    SecureDocsOnlyShareWithinOneMunicipality = 19,
}

impl WebResponseStatusSubCode {
    /// Try to convert a raw sub-code integer to a known variant.
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            1 => Some(Self::AuthorizationDeniedAnyScope),
            2 => Some(Self::AuthorizationDeniedInstitutionScope),
            3 => Some(Self::AuthorizationDeniedGroupScope),
            4 => Some(Self::AuthorizationDeniedProfileScope),
            5 => Some(Self::AuthorizationDeniedBlockedCommunication),
            6 => Some(Self::AuthorizationDeniedAccessNotGranted),
            7 => Some(Self::AuthorizationDeniedUserDeactivated),
            8 => Some(Self::AuthorizationStepUpRequired),
            9 => Some(Self::InvalidToken),
            10 => Some(Self::OutOfSyncPresenceConfiguration),
            11 => Some(Self::UnregisterDeviceFailed),
            12 => Some(Self::CrossMunicipalityTagging),
            13 => Some(Self::SessionExpired),
            14 => Some(Self::ExceedingMaximumParticipants),
            15 => Some(Self::DateAlreadyHasOccurrenceFromSameSeries),
            16 => Some(Self::FirstRepeatingEventExceptionOutOfRange),
            17 => Some(Self::LastRepeatingEventExceptionOutOfRange),
            18 => Some(Self::DeactivatedInstitutionProfile),
            19 => Some(Self::SecureDocsOnlyShareWithinOneMunicipality),
            _ => None,
        }
    }
}

/// Generic API response envelope.
///
/// Maps to `AulaServiceResponse<T>` from the decompiled assembly.
/// Most API endpoints return this wrapper around their actual payload.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaServiceResponse<T> {
    /// The response status block.
    pub status: WebResponseStatus,

    /// The actual response payload.
    pub data: T,
}

/// Array response wrapper with pagination metadata.
///
/// Maps to `DataArrayResponse<T>` from the decompiled assembly.
/// Used by endpoints that return lists/collections.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataArrayResponse<T> {
    /// Total number of matching results (may exceed `results.len()`).
    pub total_hits: i32,

    /// The result items for this page.
    pub results: Vec<T>,
}

/// Error response wrapper.
///
/// Maps to `AulaErrorResponseWrapper<T>` from the decompiled assembly.
/// Returned when the API responds with an error condition.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaErrorResponse<T> {
    /// Error status with typed error information.
    pub status: AulaErrorResponseStatus<T>,
}

/// Error status block with typed error information.
///
/// Maps to `AulaErrorResponseWrapperStatus<T>` from the decompiled assembly.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaErrorResponseStatus<T> {
    /// Typed error details.
    pub error_information: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_service_response() {
        let json = r#"{
            "status": {
                "httpCode": 200,
                "code": 0,
                "message": null,
                "presentedMessage": null,
                "subCode": null,
                "htmlContentIfError": null
            },
            "data": {"name": "test"}
        }"#;

        let resp: AulaServiceResponse<serde_json::Value> =
            serde_json::from_str(json).expect("should deserialize");
        assert_eq!(resp.status.http_code, 200);
        assert_eq!(resp.data["name"], "test");
    }

    #[test]
    fn deserialize_data_array_response() {
        let json = r#"{
            "totalHits": 2,
            "results": [1, 2]
        }"#;

        let resp: DataArrayResponse<i32> = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(resp.total_hits, 2);
        assert_eq!(resp.results, vec![1, 2]);
    }

    #[test]
    fn deserialize_status_with_sub_code() {
        let json = r#"{
            "httpCode": 403,
            "code": 100,
            "message": "access denied",
            "subCode": 8
        }"#;

        let status: WebResponseStatus = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(status.http_code, 403);
        assert_eq!(status.sub_code, Some(8));
        assert_eq!(
            WebResponseStatusSubCode::from_code(status.sub_code.unwrap()),
            Some(WebResponseStatusSubCode::AuthorizationStepUpRequired)
        );
    }

    #[test]
    fn deserialize_minimal_status() {
        // All fields optional/defaulted; real API often omits httpCode
        let json = r#"{"code": 0, "message": "OK"}"#;
        let status: WebResponseStatus = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(status.http_code, 0);
        assert_eq!(status.backend_error_code, 0);
        assert_eq!(status.message.as_deref(), Some("OK"));
        assert!(status.sub_code.is_none());
    }

    #[test]
    fn sub_code_from_code_known() {
        assert_eq!(
            WebResponseStatusSubCode::from_code(13),
            Some(WebResponseStatusSubCode::SessionExpired)
        );
    }

    #[test]
    fn sub_code_from_code_unknown() {
        assert_eq!(WebResponseStatusSubCode::from_code(999), None);
    }

    #[test]
    fn deserialize_error_response() {
        let json = r#"{
            "status": {
                "errorInformation": "something went wrong"
            }
        }"#;

        let resp: AulaErrorResponse<String> =
            serde_json::from_str(json).expect("should deserialize");
        assert_eq!(resp.status.error_information, "something went wrong");
    }
}
