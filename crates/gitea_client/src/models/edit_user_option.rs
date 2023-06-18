/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditUserOption : EditUserOption edit user options



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditUserOption {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "allow_create_organization", skip_serializing_if = "Option::is_none")]
    pub allow_create_organization: Option<bool>,
    #[serde(rename = "allow_git_hook", skip_serializing_if = "Option::is_none")]
    pub allow_git_hook: Option<bool>,
    #[serde(rename = "allow_import_local", skip_serializing_if = "Option::is_none")]
    pub allow_import_local: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "login_name")]
    pub login_name: String,
    #[serde(rename = "max_repo_creation", skip_serializing_if = "Option::is_none")]
    pub max_repo_creation: Option<i64>,
    #[serde(rename = "must_change_password", skip_serializing_if = "Option::is_none")]
    pub must_change_password: Option<bool>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "prohibit_login", skip_serializing_if = "Option::is_none")]
    pub prohibit_login: Option<bool>,
    #[serde(rename = "restricted", skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "source_id")]
    pub source_id: i64,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl EditUserOption {
    /// EditUserOption edit user options
    pub fn new(login_name: String, source_id: i64) -> EditUserOption {
        EditUserOption {
            active: None,
            admin: None,
            allow_create_organization: None,
            allow_git_hook: None,
            allow_import_local: None,
            description: None,
            email: None,
            full_name: None,
            location: None,
            login_name,
            max_repo_creation: None,
            must_change_password: None,
            password: None,
            prohibit_login: None,
            restricted: None,
            source_id,
            visibility: None,
            website: None,
        }
    }
}


