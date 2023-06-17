/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TimelineComment : TimelineComment represents a timeline comment (comment of any type) on a commit or issue



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimelineComment {
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<crate::models::User>>,
    #[serde(rename = "assignee_team", skip_serializing_if = "Option::is_none")]
    pub assignee_team: Option<Box<crate::models::Team>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dependent_issue", skip_serializing_if = "Option::is_none")]
    pub dependent_issue: Option<Box<crate::models::Issue>>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issue_url", skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<crate::models::Label>>,
    #[serde(rename = "milestone", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Box<crate::models::Milestone>>,
    #[serde(rename = "new_ref", skip_serializing_if = "Option::is_none")]
    pub new_ref: Option<String>,
    #[serde(rename = "new_title", skip_serializing_if = "Option::is_none")]
    pub new_title: Option<String>,
    #[serde(rename = "old_milestone", skip_serializing_if = "Option::is_none")]
    pub old_milestone: Option<Box<crate::models::Milestone>>,
    #[serde(rename = "old_project_id", skip_serializing_if = "Option::is_none")]
    pub old_project_id: Option<i64>,
    #[serde(rename = "old_ref", skip_serializing_if = "Option::is_none")]
    pub old_ref: Option<String>,
    #[serde(rename = "old_title", skip_serializing_if = "Option::is_none")]
    pub old_title: Option<String>,
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(rename = "pull_request_url", skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<String>,
    #[serde(rename = "ref_action", skip_serializing_if = "Option::is_none")]
    pub ref_action: Option<String>,
    #[serde(rename = "ref_comment", skip_serializing_if = "Option::is_none")]
    pub ref_comment: Option<Box<crate::models::Comment>>,
    /// commit SHA where issue/PR was referenced
    #[serde(rename = "ref_commit_sha", skip_serializing_if = "Option::is_none")]
    pub ref_commit_sha: Option<String>,
    #[serde(rename = "ref_issue", skip_serializing_if = "Option::is_none")]
    pub ref_issue: Option<Box<crate::models::Issue>>,
    /// whether the assignees were removed or added
    #[serde(rename = "removed_assignee", skip_serializing_if = "Option::is_none")]
    pub removed_assignee: Option<bool>,
    #[serde(rename = "resolve_doer", skip_serializing_if = "Option::is_none")]
    pub resolve_doer: Option<Box<crate::models::User>>,
    #[serde(rename = "review_id", skip_serializing_if = "Option::is_none")]
    pub review_id: Option<i64>,
    #[serde(rename = "tracked_time", skip_serializing_if = "Option::is_none")]
    pub tracked_time: Option<Box<crate::models::TrackedTime>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl TimelineComment {
    /// TimelineComment represents a timeline comment (comment of any type) on a commit or issue
    pub fn new() -> TimelineComment {
        TimelineComment {
            assignee: None,
            assignee_team: None,
            body: None,
            created_at: None,
            dependent_issue: None,
            html_url: None,
            id: None,
            issue_url: None,
            label: None,
            milestone: None,
            new_ref: None,
            new_title: None,
            old_milestone: None,
            old_project_id: None,
            old_ref: None,
            old_title: None,
            project_id: None,
            pull_request_url: None,
            ref_action: None,
            ref_comment: None,
            ref_commit_sha: None,
            ref_issue: None,
            removed_assignee: None,
            resolve_doer: None,
            review_id: None,
            tracked_time: None,
            r#type: None,
            updated_at: None,
            user: None,
        }
    }
}


