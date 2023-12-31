/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationPermissions : OrganizationPermissions list different users permissions on an organization



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationPermissions {
    #[serde(rename = "can_create_repository", skip_serializing_if = "Option::is_none")]
    pub can_create_repository: Option<bool>,
    #[serde(rename = "can_read", skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "can_write", skip_serializing_if = "Option::is_none")]
    pub can_write: Option<bool>,
    #[serde(rename = "is_admin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(rename = "is_owner", skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
}

impl OrganizationPermissions {
    /// OrganizationPermissions list different users permissions on an organization
    pub fn new() -> OrganizationPermissions {
        OrganizationPermissions {
            can_create_repository: None,
            can_read: None,
            can_write: None,
            is_admin: None,
            is_owner: None,
        }
    }
}


