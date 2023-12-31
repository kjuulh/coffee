/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Identity : Identity for a person's identity like an author or committer



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Identity {
    /// Identity for a person's identity like an author or committer
    pub fn new() -> Identity {
        Identity {
            email: None,
            name: None,
        }
    }
}


