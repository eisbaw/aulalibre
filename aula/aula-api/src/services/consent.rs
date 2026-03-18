//! Consent service.
//!
//! Maps to `AulaNative.Services.Web.ConsentWebService` from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_CONSENTS` | `consents.getConsentResponses` |
//! | `POST_CONSENTS` | `consents.updateConsentResponses` |

use crate::models::consent::{InstitutionProfileConsentDto, ProfileConsentUpdatesDto};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get consent status for all institution profiles.
///
/// # Endpoint
///
/// `GET ?method=consents.getConsentResponses`
pub async fn get_consents(
    session: &mut Session,
) -> crate::Result<Vec<InstitutionProfileConsentDto>> {
    session.get("?method=consents.getConsentResponses").await
}

/// Submit consent responses for an institution profile.
///
/// # Endpoint
///
/// `POST ?method=consents.updateConsentResponses`
pub async fn post_consents(
    session: &mut Session,
    updates: &ProfileConsentUpdatesDto,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=consents.updateConsentResponses", updates)
        .await
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
