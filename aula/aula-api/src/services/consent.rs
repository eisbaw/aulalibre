//! Consent service.
//!
//! Maps to `AulaNative.Services.Web.ConsentWebService` (2 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.19.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_consents` | GET | `/consents` |
//! | `post_consents` | POST | `/consents` |

use crate::models::consent::{InstitutionProfileConsentDto, ProfileConsentUpdatesDto};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get consent status for all institution profiles.
///
/// Maps to `ConsentWebService.GetConsents()`.
///
/// # Endpoint (inferred)
///
/// `GET /consents`
pub async fn get_consents(
    session: &mut Session,
) -> crate::Result<Vec<InstitutionProfileConsentDto>> {
    session.get("consents").await
}

/// Submit consent responses for an institution profile.
///
/// Maps to `ConsentWebService.PostConsents()`.
///
/// # Endpoint (inferred)
///
/// `POST /consents`
pub async fn post_consents(
    session: &mut Session,
    updates: &ProfileConsentUpdatesDto,
) -> crate::Result<serde_json::Value> {
    session.post("consents", updates).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::models::consent::{
        ConsentUpdateDto, InstitutionProfileConsentDto, ProfileConsentUpdatesDto,
    };

    #[test]
    fn consent_list_deserializes() {
        let json = r#"[
            {
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
                        "consentDescription": "Billeder af barnet",
                        "consentResponseAnswer": "Accepted",
                        "consentResponseStatus": "Active",
                        "editable": true,
                        "viewOnlyDependency": null,
                        "viewOrder": 1,
                        "fromDate": null,
                        "toDate": null
                    }
                ]
            }
        ]"#;
        let consents: Vec<InstitutionProfileConsentDto> = serde_json::from_str(json).unwrap();
        assert_eq!(consents.len(), 1);
        assert_eq!(consents[0].consent_responses.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn consent_update_serializes() {
        let updates = ProfileConsentUpdatesDto {
            institution_profile_id: Some(42),
            institution_profile_consent_updates: Some(vec![ConsentUpdateDto {
                consent_id: Some(10),
                answer: Some("Accepted".into()),
            }]),
        };
        let json = serde_json::to_value(&updates).unwrap();
        assert_eq!(json["institutionProfileId"], 42);
        let updates_arr = json["institutionProfileConsentUpdates"].as_array().unwrap();
        assert_eq!(updates_arr.len(), 1);
        assert_eq!(updates_arr[0]["answer"], "Accepted");
    }
}
