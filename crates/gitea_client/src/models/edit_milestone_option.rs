/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditMilestoneOption : EditMilestoneOption options for editing a milestone



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditMilestoneOption {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "due_on", skip_serializing_if = "Option::is_none")]
    pub due_on: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl EditMilestoneOption {
    /// EditMilestoneOption options for editing a milestone
    pub fn new() -> EditMilestoneOption {
        EditMilestoneOption {
            description: None,
            due_on: None,
            state: None,
            title: None,
        }
    }
}


