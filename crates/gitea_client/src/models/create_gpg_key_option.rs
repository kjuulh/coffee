/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateGpgKeyOption : CreateGPGKeyOption options create user GPG key

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGpgKeyOption {
  /// An armored GPG key to add
  #[serde(rename = "armored_public_key")]
  armored_public_key: String,
  #[serde(rename = "armored_signature")]
  armored_signature: Option<String>
}

impl CreateGpgKeyOption {
  /// CreateGPGKeyOption options create user GPG key
  pub fn new(armored_public_key: String) -> CreateGpgKeyOption {
    CreateGpgKeyOption {
      armored_public_key: armored_public_key,
      armored_signature: None
    }
  }

  pub fn set_armored_public_key(&mut self, armored_public_key: String) {
    self.armored_public_key = armored_public_key;
  }

  pub fn with_armored_public_key(mut self, armored_public_key: String) -> CreateGpgKeyOption {
    self.armored_public_key = armored_public_key;
    self
  }

  pub fn armored_public_key(&self) -> &String {
    &self.armored_public_key
  }


  pub fn set_armored_signature(&mut self, armored_signature: String) {
    self.armored_signature = Some(armored_signature);
  }

  pub fn with_armored_signature(mut self, armored_signature: String) -> CreateGpgKeyOption {
    self.armored_signature = Some(armored_signature);
    self
  }

  pub fn armored_signature(&self) -> Option<&String> {
    self.armored_signature.as_ref()
  }

  pub fn reset_armored_signature(&mut self) {
    self.armored_signature = None;
  }

}



