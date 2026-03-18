//! Personal reference data service.
//!
//! Maps to `AulaNative.Services.Web.PersonalReferenceDataWebService` (3 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_PERSONAL_REFERENCE_DATA_QUESTION` | `personalReferenceData.getPersonalReferenceDataQuestion` |
//! | `GET_PERSONAL_REFERENCE_DATA_ANSWER` | `personalReferenceData.getPersonalReferenceDataAdditionalDataAnswer` |
//! | `GET_PERSONAL_REFERENCE_DATA_CONSENT_ANSWER` | `personalReferenceData.getPersonalReferenceDataConsentAnswer` |

use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get additional answer data for personal references.
///
/// Maps to `PersonalReferenceDataWebService.GetPersonalReferenceAdditionalAnswerData()`.
///
/// # Endpoint
///
/// `GET ?method=personalReferenceData.getPersonalReferenceDataAdditionalDataAnswer`
pub async fn get_additional_answer_data(session: &mut Session) -> crate::Result<serde_json::Value> {
    session
        .get("?method=personalReferenceData.getPersonalReferenceDataAdditionalDataAnswer")
        .await
}

/// Get consent answer data for personal references.
///
/// Maps to `PersonalReferenceDataWebService.GetPersonalReferenceConsentAnswerData()`.
///
/// # Endpoint
///
/// `GET ?method=personalReferenceData.getPersonalReferenceDataConsentAnswer`
pub async fn get_consent_answer_data(session: &mut Session) -> crate::Result<serde_json::Value> {
    session
        .get("?method=personalReferenceData.getPersonalReferenceDataConsentAnswer")
        .await
}

/// Get question data for personal references.
///
/// Maps to `PersonalReferenceDataWebService.GetPersonalReferenceQuestionData()`.
///
/// # Endpoint
///
/// `GET ?method=personalReferenceData.getPersonalReferenceDataQuestion`
pub async fn get_question_data(session: &mut Session) -> crate::Result<serde_json::Value> {
    session
        .get("?method=personalReferenceData.getPersonalReferenceDataQuestion")
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn personal_reference_json_value_deserializes() {
        // Since the actual response types are unknown, we just verify
        // that arbitrary JSON can be deserialized to serde_json::Value.
        let json = r#"{"data": [{"questionId": 1, "text": "Bemærkninger"}]}"#;
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(v["data"].is_array());
    }
}
