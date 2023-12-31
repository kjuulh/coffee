/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ActivityPub : ActivityPub type



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActivityPub {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<String>,
}

impl ActivityPub {
    /// ActivityPub type
    pub fn new() -> ActivityPub {
        ActivityPub {
            at_context: None,
        }
    }
}


