/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Comment : Comment represents a comment on a commit or issue



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<crate::models::Attachment>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issue_url", skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<String>,
    #[serde(rename = "original_author", skip_serializing_if = "Option::is_none")]
    pub original_author: Option<String>,
    #[serde(rename = "original_author_id", skip_serializing_if = "Option::is_none")]
    pub original_author_id: Option<i64>,
    #[serde(rename = "pull_request_url", skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl Comment {
    /// Comment represents a comment on a commit or issue
    pub fn new() -> Comment {
        Comment {
            assets: None,
            body: None,
            created_at: None,
            html_url: None,
            id: None,
            issue_url: None,
            original_author: None,
            original_author_id: None,
            pull_request_url: None,
            updated_at: None,
            user: None,
        }
    }
}


