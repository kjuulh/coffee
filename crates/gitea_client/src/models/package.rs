/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Package : Package represents a package

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "creator")]
  creator: Option<::models::User>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "owner")]
  owner: Option<::models::User>,
  #[serde(rename = "repository")]
  repository: Option<::models::Repository>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "version")]
  version: Option<String>
}

impl Package {
  /// Package represents a package
  pub fn new() -> Package {
    Package {
      created_at: None,
      creator: None,
      id: None,
      name: None,
      owner: None,
      repository: None,
      _type: None,
      version: None
    }
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> Package {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_creator(&mut self, creator: ::models::User) {
    self.creator = Some(creator);
  }

  pub fn with_creator(mut self, creator: ::models::User) -> Package {
    self.creator = Some(creator);
    self
  }

  pub fn creator(&self) -> Option<&::models::User> {
    self.creator.as_ref()
  }

  pub fn reset_creator(&mut self) {
    self.creator = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Package {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Package {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_owner(&mut self, owner: ::models::User) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: ::models::User) -> Package {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&::models::User> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

  pub fn set_repository(&mut self, repository: ::models::Repository) {
    self.repository = Some(repository);
  }

  pub fn with_repository(mut self, repository: ::models::Repository) -> Package {
    self.repository = Some(repository);
    self
  }

  pub fn repository(&self) -> Option<&::models::Repository> {
    self.repository.as_ref()
  }

  pub fn reset_repository(&mut self) {
    self.repository = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> Package {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> Package {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

}



