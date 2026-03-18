//! User-level models.
//!
//! Maps to `Models.Users` namespace from the decompiled assembly.
//! See `data_models.md` section "Models.Users".

use serde::{Deserialize, Serialize};

use super::institutions::Institution;
use super::profiles::ProfileId;

/// User profile context containing institution memberships.
///
/// Maps to `Models.Users.ProfileContext`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileContext {
    pub profile_id: Option<ProfileId>,
    pub portal_role: Option<String>,
    pub institutions: Option<Vec<Institution>>,
}

/// User search result model.
///
/// Maps to `Models.Users.User`.
/// Note: this is a search-oriented view, not the full user identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub address: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub highlights: Option<Vec<serde_json::Value>>,
}

/// Relationship between a user and children/institutions.
///
/// Maps to `Models.Users.UserRelationship`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRelationship {
    pub profile_id: Option<ProfileId>,
    pub child_relationships: Option<Vec<String>>,
    pub institution_relationships: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_profile_context() {
        let json = r#"{
            "profileId": 100,
            "portalRole": "Guardian",
            "institutions": []
        }"#;
        let pc: ProfileContext = serde_json::from_str(json).unwrap();
        assert_eq!(pc.profile_id, Some(100));
        assert_eq!(pc.portal_role.as_deref(), Some("Guardian"));
    }

    #[test]
    fn deserialize_user() {
        let json = r#"{
            "address": "Vestergade 12, 8000 Aarhus",
            "displayName": "Lars Hansen",
            "email": "lars@example.dk",
            "firstName": "Lars",
            "lastName": "Hansen",
            "highlights": []
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.display_name.as_deref(), Some("Lars Hansen"));
        assert_eq!(user.first_name.as_deref(), Some("Lars"));
    }

    #[test]
    fn deserialize_user_relationship() {
        let json = r#"{
            "profileId": 50,
            "childRelationships": ["child1", "child2"],
            "institutionRelationships": ["inst1"]
        }"#;
        let ur: UserRelationship = serde_json::from_str(json).unwrap();
        assert_eq!(ur.profile_id, Some(50));
        assert_eq!(ur.child_relationships.as_ref().unwrap().len(), 2);
    }
}
