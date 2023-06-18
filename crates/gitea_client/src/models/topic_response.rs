/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TopicResponse : TopicResponse for returning topics



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TopicResponse {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "repo_count", skip_serializing_if = "Option::is_none")]
    pub repo_count: Option<i64>,
    #[serde(rename = "topic_name", skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl TopicResponse {
    /// TopicResponse for returning topics
    pub fn new() -> TopicResponse {
        TopicResponse {
            created: None,
            id: None,
            repo_count: None,
            topic_name: None,
            updated: None,
        }
    }
}


