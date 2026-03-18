//! Additional master data service.
//!
//! Maps to `AulaNative.Services.Web.AdditionalMasterDataService` from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.4.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_additional_master_data` | GET | `/additionalMasterData` |
//! | `get_by_institution_profile_id` | GET | `/additionalMasterData/{id}` |
//! | `post_additional_master_data` | POST | `/additionalMasterData` |
//! | `post_additional_master_data_employee` | POST | `/additionalMasterData/employee` |

use serde::{Deserialize, Serialize};

use crate::models::profiles::InstitutionProfileId;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response / request types specific to this service
// ---------------------------------------------------------------------------

/// Additional master data entry.
///
/// Represents extra profile information beyond the core profile fields
/// (e.g., dietary restrictions, medical info, custom institution fields).
///
/// NOTE: The exact field set is inferred from the method signatures.
/// The actual response may contain different or additional fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalMasterData {
    /// The institution profile this data belongs to.
    pub institution_profile_id: Option<InstitutionProfileId>,
    /// Key-value pairs or structured data.
    /// Using `serde_json::Value` because the exact schema is unknown.
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Request body for updating additional master data.
///
/// Inferred from `AdditionalMasterDataService.PostAdditionalMasterData()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdditionalMasterDataRequest {
    /// The institution profile to update data for.
    pub institution_profile_id: InstitutionProfileId,
    /// The data payload to submit.
    pub data: serde_json::Value,
}

/// Request body for updating employee-specific additional master data.
///
/// Inferred from `AdditionalMasterDataService.PostAdditionalMasterDataEmployee()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdditionalMasterDataEmployeeRequest {
    /// The institution profile to update data for.
    pub institution_profile_id: InstitutionProfileId,
    /// The employee-specific data payload.
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get additional master data for the current user.
///
/// Maps to `AdditionalMasterDataService.GetAdditionalMasterData()`.
///
/// # Endpoint (inferred)
///
/// `GET /additionalMasterData`
pub async fn get_additional_master_data(
    session: &mut Session,
) -> crate::Result<Vec<AdditionalMasterData>> {
    session.get("additionalMasterData").await
}

/// Get additional master data for a specific institution profile.
///
/// Maps to `AdditionalMasterDataService.GetAdditionalMasterDataByInstitutionProfileId()`.
///
/// # Endpoint (inferred)
///
/// `GET /additionalMasterData/{institution_profile_id}`
pub async fn get_by_institution_profile_id(
    session: &mut Session,
    institution_profile_id: InstitutionProfileId,
) -> crate::Result<AdditionalMasterData> {
    session
        .get(&format!("additionalMasterData/{institution_profile_id}"))
        .await
}

/// Update additional master data.
///
/// Maps to `AdditionalMasterDataService.PostAdditionalMasterData()`.
///
/// # Endpoint (inferred)
///
/// `POST /additionalMasterData`
pub async fn post_additional_master_data(
    session: &mut Session,
    request: &UpdateAdditionalMasterDataRequest,
) -> crate::Result<serde_json::Value> {
    session.post("additionalMasterData", request).await
}

/// Update employee-specific additional master data.
///
/// Maps to `AdditionalMasterDataService.PostAdditionalMasterDataEmployee()`.
///
/// # Endpoint (inferred)
///
/// `POST /additionalMasterData/employee`
pub async fn post_additional_master_data_employee(
    session: &mut Session,
    request: &UpdateAdditionalMasterDataEmployeeRequest,
) -> crate::Result<serde_json::Value> {
    session.post("additionalMasterData/employee", request).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn additional_master_data_deserializes() {
        let json_str = r#"{
            "institutionProfileId": 42,
            "data": {"dietaryRestrictions": "gluten-free", "notes": "allergies"}
        }"#;
        let d: AdditionalMasterData = serde_json::from_str(json_str).unwrap();
        assert_eq!(d.institution_profile_id, Some(42));
        assert_eq!(d.data["dietaryRestrictions"], "gluten-free");
    }

    #[test]
    fn additional_master_data_defaults() {
        let json_str = r#"{}"#;
        let d: AdditionalMasterData = serde_json::from_str(json_str).unwrap();
        assert!(d.institution_profile_id.is_none());
        assert!(d.data.is_null());
    }

    #[test]
    fn update_request_serializes() {
        let req = UpdateAdditionalMasterDataRequest {
            institution_profile_id: 42,
            data: json!({"key": "value"}),
        };
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["institutionProfileId"], 42);
        assert_eq!(v["data"]["key"], "value");
    }

    #[test]
    fn employee_update_request_serializes() {
        let req = UpdateAdditionalMasterDataEmployeeRequest {
            institution_profile_id: 99,
            data: json!({"role": "teacher"}),
        };
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["institutionProfileId"], 99);
        assert_eq!(v["data"]["role"], "teacher");
    }
}
