/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Reaction : Reaction contain one reaction

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
  #[serde(rename = "content")]
  content: Option<String>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "user")]
  user: Option<::models::User>
}

impl Reaction {
  /// Reaction contain one reaction
  pub fn new() -> Reaction {
    Reaction {
      content: None,
      created_at: None,
      user: None
    }
  }

  pub fn set_content(&mut self, content: String) {
    self.content = Some(content);
  }

  pub fn with_content(mut self, content: String) -> Reaction {
    self.content = Some(content);
    self
  }

  pub fn content(&self) -> Option<&String> {
    self.content.as_ref()
  }

  pub fn reset_content(&mut self) {
    self.content = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> Reaction {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_user(&mut self, user: ::models::User) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::User) -> Reaction {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::User> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



