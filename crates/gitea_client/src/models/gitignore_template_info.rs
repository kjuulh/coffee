/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GitignoreTemplateInfo : GitignoreTemplateInfo name and text of a gitignore template



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GitignoreTemplateInfo {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl GitignoreTemplateInfo {
    /// GitignoreTemplateInfo name and text of a gitignore template
    pub fn new() -> GitignoreTemplateInfo {
        GitignoreTemplateInfo {
            name: None,
            source: None,
        }
    }
}


