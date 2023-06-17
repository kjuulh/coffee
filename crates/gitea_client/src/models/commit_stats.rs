/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommitStats : CommitStats is statistics for a RepoCommit



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommitStats {
    #[serde(rename = "additions", skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    #[serde(rename = "deletions", skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl CommitStats {
    /// CommitStats is statistics for a RepoCommit
    pub fn new() -> CommitStats {
        CommitStats {
            additions: None,
            deletions: None,
            total: None,
        }
    }
}


