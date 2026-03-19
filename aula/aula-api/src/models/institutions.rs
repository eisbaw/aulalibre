//! Institution and administrative authority models.
//!
//! Maps to `Models.Institutions` namespace from the decompiled assembly.
//! See `data_models.md` section "Models.Institutions".

use serde::{Deserialize, Serialize};

use crate::enums::profiles::{InstitutionRole, InstitutionTypeEnum};

use super::groups::Group;
use super::profiles::{ChildProfile, InstitutionCode, InstitutionProfileId, Permission};

// ---------------------------------------------------------------------------
// Administrative authority
// ---------------------------------------------------------------------------

/// Administrative authority (Danish: "forvaltning") governing institutions.
///
/// Maps to `Models.Institutions.AdministrativeAuthority`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrativeAuthority {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_codes: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Institution identity (lightweight reference)
// ---------------------------------------------------------------------------

/// Lightweight institution reference used inside profiles.
///
/// Maps to `Models.Institutions.InstitutionIdentity`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionIdentity {
    pub institution_code: Option<InstitutionCode>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub administrative_authority: Option<AdministrativeAuthority>,
}

/// Minimal institution reference (name + code only).
///
/// Maps to `Models.Institutions.SimpleInstitution`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleInstitution {
    pub institution_name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
}

// ---------------------------------------------------------------------------
// Full institution model
// ---------------------------------------------------------------------------

/// Full institution with children, permissions, groups, and metadata.
///
/// Maps to `Models.Institutions.Institution`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Institution {
    pub children: Option<Vec<ChildProfile>>,
    pub institution_profile_id: Option<InstitutionProfileId>,
    pub name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
    pub institution_type: Option<InstitutionTypeEnum>,
    pub municipality_code: Option<String>,
    pub institution_role: Option<InstitutionRole>,
    pub permissions: Option<Vec<Permission>>,
    pub groups: Option<Vec<Group>>,
    pub administrative_authority: Option<AdministrativeAuthority>,
    #[serde(default)]
    pub communication_block: bool,
    #[serde(default)]
    pub selected: bool,
    pub mailbox_id: Option<i32>,
    pub short_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_administrative_authority() {
        let json = r#"{
            "id": 1,
            "name": "Aarhus Kommune",
            "institutionCodes": ["101001", "101002"]
        }"#;
        let aa: AdministrativeAuthority = serde_json::from_str(json).unwrap();
        assert_eq!(aa.id, Some(1));
        assert_eq!(aa.institution_codes.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_institution_identity() {
        let json = r#"{
            "institutionCode": "101001",
            "institutionName": "Viby Skole",
            "municipalityCode": "751",
            "municipalityName": "Aarhus",
            "administrativeAuthority": null
        }"#;
        let ii: InstitutionIdentity = serde_json::from_str(json).unwrap();
        assert_eq!(ii.institution_code.as_deref(), Some("101001"));
        assert_eq!(ii.institution_name.as_deref(), Some("Viby Skole"));
    }

    #[test]
    fn deserialize_institution() {
        let json = r#"{
            "children": [],
            "institutionProfileId": 42,
            "name": "Viby Skole",
            "institutionCode": "101001",
            "institutionType": "School",
            "municipalityCode": "751",
            "institutionRole": "guardian",
            "permissions": [],
            "groups": [],
            "administrativeAuthority": null,
            "communicationBlock": false,
            "selected": true,
            "mailboxId": 5,
            "shortName": "VS"
        }"#;
        let inst: Institution = serde_json::from_str(json).unwrap();
        assert_eq!(inst.institution_profile_id, Some(42));
        assert_eq!(inst.institution_type, Some(InstitutionTypeEnum::School));
        assert!(inst.selected);
    }

    #[test]
    fn deserialize_simple_institution() {
        let json = r#"{
            "institutionName": "Viby Skole",
            "institutionCode": "101001"
        }"#;
        let si: SimpleInstitution = serde_json::from_str(json).unwrap();
        assert_eq!(si.institution_name.as_deref(), Some("Viby Skole"));
    }
}
