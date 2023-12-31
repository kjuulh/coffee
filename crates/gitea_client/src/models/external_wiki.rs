/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.20.0+rc0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExternalWiki : ExternalWiki represents setting for external wiki



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalWiki {
    /// URL of external wiki.
    #[serde(rename = "external_wiki_url", skip_serializing_if = "Option::is_none")]
    pub external_wiki_url: Option<String>,
}

impl ExternalWiki {
    /// ExternalWiki represents setting for external wiki
    pub fn new() -> ExternalWiki {
        ExternalWiki {
            external_wiki_url: None,
        }
    }
}


