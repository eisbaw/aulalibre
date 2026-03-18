//! Consent domain models.
//!
//! These types represent consent management in Aula, where guardians
//! grant/decline various data-sharing consents per child/institution profile.
//!
//! See `data_models.md` DTOs.Consent and Models.Consents.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Consent response types (from GET /consents)
// ---------------------------------------------------------------------------

/// Individual consent response with answer and metadata.
///
/// Maps to `Models.Consents.ConsentResponsesDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentResponsesDto {
    pub id: Option<i64>,
    pub consent_id: Option<i32>,
    pub allowed_answers: Option<Vec<String>>,
    pub consent_description: Option<String>,
    pub consent_response_answer: Option<String>,
    pub consent_response_status: Option<String>,
    #[serde(default)]
    pub editable: bool,
    pub view_only_dependency: Option<i32>,
    pub view_order: Option<i32>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

/// Consent profile with institution profile reference and consent responses.
///
/// Maps to `Models.Consents.InstitutionProfileConsentDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionProfileConsentDto {
    pub institution_profile: Option<InstitutionProfileConsent>,
    pub consent_responses: Option<Vec<ConsentResponsesDto>>,
}

/// Minimal institution profile reference used in consent context.
///
/// Maps to `Models.Consents.InstitutionProfileConsent` (inferred from usage).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionProfileConsent {
    pub institution_profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Consent update types (for PUT /consents)
// ---------------------------------------------------------------------------

/// Single consent answer update.
///
/// Maps to `DTOs.Consent.ConsentUpdateDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentUpdateDto {
    pub consent_id: Option<i64>,
    pub answer: Option<String>,
}

/// Batch consent update for a specific institution profile.
///
/// Maps to `DTOs.Consent.ProfileConsentUpdatesDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileConsentUpdatesDto {
    pub institution_profile_id: Option<i64>,
    pub institution_profile_consent_updates: Option<Vec<ConsentUpdateDto>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_consent_response() {
        let json = r#"{
            "id": 1,
            "consentId": 10,
            "allowedAnswers": ["Accepted", "Declined"],
            "consentDescription": "Billeder af barnet",
            "consentResponseAnswer": "Accepted",
            "consentResponseStatus": "Active",
            "editable": true,
            "viewOnlyDependency": null,
            "viewOrder": 1,
            "fromDate": "2025-08-01",
            "toDate": null
        }"#;
        let c: ConsentResponsesDto = serde_json::from_str(json).unwrap();
        assert_eq!(c.consent_id, Some(10));
        assert!(c.editable);
        assert_eq!(c.consent_response_answer.as_deref(), Some("Accepted"));
    }

    #[test]
    fn deserialize_institution_profile_consent() {
        let json = r#"{
            "institutionProfile": {
                "institutionProfileId": 42,
                "firstName": "Anna",
                "lastName": "Jensen",
                "fullName": "Anna Jensen",
                "institutionCode": "101001",
                "institutionName": "Viby Skole"
            },
            "consentResponses": [
                {
                    "id": 1,
                    "consentId": 10,
                    "allowedAnswers": ["Accepted", "Declined"],
                    "consentDescription": "Test",
                    "consentResponseAnswer": null,
                    "consentResponseStatus": "Pending",
                    "editable": true,
                    "viewOnlyDependency": null,
                    "viewOrder": 1,
                    "fromDate": null,
                    "toDate": null
                }
            ]
        }"#;
        let c: InstitutionProfileConsentDto = serde_json::from_str(json).unwrap();
        assert_eq!(
            c.institution_profile
                .as_ref()
                .unwrap()
                .institution_profile_id,
            Some(42)
        );
        assert_eq!(c.consent_responses.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn deserialize_consent_update() {
        let json = r#"{
            "institutionProfileId": 42,
            "institutionProfileConsentUpdates": [
                {"consentId": 10, "answer": "Accepted"},
                {"consentId": 11, "answer": "Declined"}
            ]
        }"#;
        let u: ProfileConsentUpdatesDto = serde_json::from_str(json).unwrap();
        assert_eq!(u.institution_profile_id, Some(42));
        assert_eq!(
            u.institution_profile_consent_updates
                .as_ref()
                .unwrap()
                .len(),
            2
        );
    }
}
