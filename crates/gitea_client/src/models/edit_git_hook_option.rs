/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditGitHookOption : EditGitHookOption options when modifying one Git hook



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditGitHookOption {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl EditGitHookOption {
    /// EditGitHookOption options when modifying one Git hook
    pub fn new() -> EditGitHookOption {
        EditGitHookOption {
            content: None,
        }
    }
}


