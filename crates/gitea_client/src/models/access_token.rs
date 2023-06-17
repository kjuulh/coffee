/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "token_last_eight", skip_serializing_if = "Option::is_none")]
    pub token_last_eight: Option<String>,
}

impl AccessToken {
    pub fn new() -> AccessToken {
        AccessToken {
            id: None,
            name: None,
            scopes: None,
            sha1: None,
            token_last_eight: None,
        }
    }
}


