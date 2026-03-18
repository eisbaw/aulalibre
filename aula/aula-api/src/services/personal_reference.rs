//! Personal reference data service.
//!
//! Maps to `AulaNative.Services.Web.PersonalReferenceDataWebService` (3 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.20.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_additional_answer_data` | GET | `/personalReference/additionalAnswers` |
//! | `get_consent_answer_data` | GET | `/personalReference/consentAnswers` |
//! | `get_question_data` | GET | `/personalReference/questions` |

use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get additional answer data for personal references.
///
/// Maps to `PersonalReferenceDataWebService.GetPersonalReferenceAdditionalAnswerData()`.
///
/// # Endpoint (inferred)
///
/// `GET /personalReference/additionalAnswers`
///
/// The response structure is not fully known from decompilation.
pub async fn get_additional_answer_data(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.get("personalReference/additionalAnswers").await
}

/// Get consent answer data for personal references.
///
/// Maps to `PersonalReferenceDataWebService.GetPersonalReferenceConsentAnswerData()`.
///
/// # Endpoint (inferred)
///
/// `GET /personalReference/consentAnswers`
///
/// The response structure is not fully known from decompilation.
pub async fn get_consent_answer_data(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.get("personalReference/consentAnswers").await
}

/// Get question data for personal references.
///
/// Maps to `PersonalReferenceDataWebService.GetPersonalReferenceQuestionData()`.
///
/// # Endpoint (inferred)
///
/// `GET /personalReference/questions`
///
/// The response structure is not fully known from decompilation.
pub async fn get_question_data(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.get("personalReference/questions").await
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
