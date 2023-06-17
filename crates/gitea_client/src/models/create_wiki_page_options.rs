/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateWikiPageOptions : CreateWikiPageOptions form for creating wiki



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWikiPageOptions {
    /// content must be base64 encoded
    #[serde(rename = "content_base64", skip_serializing_if = "Option::is_none")]
    pub content_base64: Option<String>,
    /// optional commit message summarizing the change
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// page title. leave empty to keep unchanged
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateWikiPageOptions {
    /// CreateWikiPageOptions form for creating wiki
    pub fn new() -> CreateWikiPageOptions {
        CreateWikiPageOptions {
            content_base64: None,
            message: None,
            title: None,
        }
    }
}


