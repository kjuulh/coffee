/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PullReviewRequestOptions : PullReviewRequestOptions are options to add or remove pull review requests



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PullReviewRequestOptions {
    #[serde(rename = "reviewers", skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<String>>,
    #[serde(rename = "team_reviewers", skip_serializing_if = "Option::is_none")]
    pub team_reviewers: Option<Vec<String>>,
}

impl PullReviewRequestOptions {
    /// PullReviewRequestOptions are options to add or remove pull review requests
    pub fn new() -> PullReviewRequestOptions {
        PullReviewRequestOptions {
            reviewers: None,
            team_reviewers: None,
        }
    }
}


