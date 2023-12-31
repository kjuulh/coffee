/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreatePullReviewOptions : CreatePullReviewOptions are options to create a pull review



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePullReviewOptions {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::models::CreatePullReviewComment>>,
    #[serde(rename = "commit_id", skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// ReviewStateType review state type
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

impl CreatePullReviewOptions {
    /// CreatePullReviewOptions are options to create a pull review
    pub fn new() -> CreatePullReviewOptions {
        CreatePullReviewOptions {
            body: None,
            comments: None,
            commit_id: None,
            event: None,
        }
    }
}


