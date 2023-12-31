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
pub struct Reference {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<crate::models::GitObject>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Reference {
    pub fn new() -> Reference {
        Reference {
            object: None,
            r#ref: None,
            url: None,
        }
    }
}


