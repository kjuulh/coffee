/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateStatusOption : CreateStatusOption holds the information needed to create a new CommitStatus for a Commit



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateStatusOption {
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// CommitStatusState holds the state of a CommitStatus It can be \"pending\", \"success\", \"error\", \"failure\", and \"warning\"
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "target_url", skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
}

impl CreateStatusOption {
    /// CreateStatusOption holds the information needed to create a new CommitStatus for a Commit
    pub fn new() -> CreateStatusOption {
        CreateStatusOption {
            context: None,
            description: None,
            state: None,
            target_url: None,
        }
    }
}


