/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineResponse2001 {
  #[serde(rename = "data")]
  data: Option<Vec<::models::User>>,
  #[serde(rename = "ok")]
  ok: Option<bool>
}

impl InlineResponse2001 {
  pub fn new() -> InlineResponse2001 {
    InlineResponse2001 {
      data: None,
      ok: None
    }
  }

  pub fn set_data(&mut self, data: Vec<::models::User>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Vec<::models::User>) -> InlineResponse2001 {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Vec<::models::User>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_ok(&mut self, ok: bool) {
    self.ok = Some(ok);
  }

  pub fn with_ok(mut self, ok: bool) -> InlineResponse2001 {
    self.ok = Some(ok);
    self
  }

  pub fn ok(&self) -> Option<&bool> {
    self.ok.as_ref()
  }

  pub fn reset_ok(&mut self) {
    self.ok = None;
  }

}



