/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeleteEmailOption : DeleteEmailOption options when deleting email addresses



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteEmailOption {
    /// email addresses to delete
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

impl DeleteEmailOption {
    /// DeleteEmailOption options when deleting email addresses
    pub fn new() -> DeleteEmailOption {
        DeleteEmailOption {
            emails: None,
        }
    }
}


