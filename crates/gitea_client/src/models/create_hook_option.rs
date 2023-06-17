/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateHookOption : CreateHookOption options when create a hook

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHookOption {
  #[serde(rename = "active")]
  active: Option<bool>,
  #[serde(rename = "authorization_header")]
  authorization_header: Option<String>,
  #[serde(rename = "branch_filter")]
  branch_filter: Option<String>,
  #[serde(rename = "config")]
  config: ::models::CreateHookOptionConfig,
  #[serde(rename = "events")]
  events: Option<Vec<String>>,
  #[serde(rename = "type")]
  _type: String
}

impl CreateHookOption {
  /// CreateHookOption options when create a hook
  pub fn new(config: ::models::CreateHookOptionConfig, _type: String) -> CreateHookOption {
    CreateHookOption {
      active: None,
      authorization_header: None,
      branch_filter: None,
      config: config,
      events: None,
      _type: _type
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> CreateHookOption {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_authorization_header(&mut self, authorization_header: String) {
    self.authorization_header = Some(authorization_header);
  }

  pub fn with_authorization_header(mut self, authorization_header: String) -> CreateHookOption {
    self.authorization_header = Some(authorization_header);
    self
  }

  pub fn authorization_header(&self) -> Option<&String> {
    self.authorization_header.as_ref()
  }

  pub fn reset_authorization_header(&mut self) {
    self.authorization_header = None;
  }

  pub fn set_branch_filter(&mut self, branch_filter: String) {
    self.branch_filter = Some(branch_filter);
  }

  pub fn with_branch_filter(mut self, branch_filter: String) -> CreateHookOption {
    self.branch_filter = Some(branch_filter);
    self
  }

  pub fn branch_filter(&self) -> Option<&String> {
    self.branch_filter.as_ref()
  }

  pub fn reset_branch_filter(&mut self) {
    self.branch_filter = None;
  }

  pub fn set_config(&mut self, config: ::models::CreateHookOptionConfig) {
    self.config = config;
  }

  pub fn with_config(mut self, config: ::models::CreateHookOptionConfig) -> CreateHookOption {
    self.config = config;
    self
  }

  pub fn config(&self) -> &::models::CreateHookOptionConfig {
    &self.config
  }


  pub fn set_events(&mut self, events: Vec<String>) {
    self.events = Some(events);
  }

  pub fn with_events(mut self, events: Vec<String>) -> CreateHookOption {
    self.events = Some(events);
    self
  }

  pub fn events(&self) -> Option<&Vec<String>> {
    self.events.as_ref()
  }

  pub fn reset_events(&mut self) {
    self.events = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> CreateHookOption {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



