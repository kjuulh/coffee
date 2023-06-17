/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChangedFile : ChangedFile store information about files affected by the pull request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangedFile {
  #[serde(rename = "additions")]
  additions: Option<i64>,
  #[serde(rename = "changes")]
  changes: Option<i64>,
  #[serde(rename = "contents_url")]
  contents_url: Option<String>,
  #[serde(rename = "deletions")]
  deletions: Option<i64>,
  #[serde(rename = "filename")]
  filename: Option<String>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "previous_filename")]
  previous_filename: Option<String>,
  #[serde(rename = "raw_url")]
  raw_url: Option<String>,
  #[serde(rename = "status")]
  status: Option<String>
}

impl ChangedFile {
  /// ChangedFile store information about files affected by the pull request
  pub fn new() -> ChangedFile {
    ChangedFile {
      additions: None,
      changes: None,
      contents_url: None,
      deletions: None,
      filename: None,
      html_url: None,
      previous_filename: None,
      raw_url: None,
      status: None
    }
  }

  pub fn set_additions(&mut self, additions: i64) {
    self.additions = Some(additions);
  }

  pub fn with_additions(mut self, additions: i64) -> ChangedFile {
    self.additions = Some(additions);
    self
  }

  pub fn additions(&self) -> Option<&i64> {
    self.additions.as_ref()
  }

  pub fn reset_additions(&mut self) {
    self.additions = None;
  }

  pub fn set_changes(&mut self, changes: i64) {
    self.changes = Some(changes);
  }

  pub fn with_changes(mut self, changes: i64) -> ChangedFile {
    self.changes = Some(changes);
    self
  }

  pub fn changes(&self) -> Option<&i64> {
    self.changes.as_ref()
  }

  pub fn reset_changes(&mut self) {
    self.changes = None;
  }

  pub fn set_contents_url(&mut self, contents_url: String) {
    self.contents_url = Some(contents_url);
  }

  pub fn with_contents_url(mut self, contents_url: String) -> ChangedFile {
    self.contents_url = Some(contents_url);
    self
  }

  pub fn contents_url(&self) -> Option<&String> {
    self.contents_url.as_ref()
  }

  pub fn reset_contents_url(&mut self) {
    self.contents_url = None;
  }

  pub fn set_deletions(&mut self, deletions: i64) {
    self.deletions = Some(deletions);
  }

  pub fn with_deletions(mut self, deletions: i64) -> ChangedFile {
    self.deletions = Some(deletions);
    self
  }

  pub fn deletions(&self) -> Option<&i64> {
    self.deletions.as_ref()
  }

  pub fn reset_deletions(&mut self) {
    self.deletions = None;
  }

  pub fn set_filename(&mut self, filename: String) {
    self.filename = Some(filename);
  }

  pub fn with_filename(mut self, filename: String) -> ChangedFile {
    self.filename = Some(filename);
    self
  }

  pub fn filename(&self) -> Option<&String> {
    self.filename.as_ref()
  }

  pub fn reset_filename(&mut self) {
    self.filename = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> ChangedFile {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_previous_filename(&mut self, previous_filename: String) {
    self.previous_filename = Some(previous_filename);
  }

  pub fn with_previous_filename(mut self, previous_filename: String) -> ChangedFile {
    self.previous_filename = Some(previous_filename);
    self
  }

  pub fn previous_filename(&self) -> Option<&String> {
    self.previous_filename.as_ref()
  }

  pub fn reset_previous_filename(&mut self) {
    self.previous_filename = None;
  }

  pub fn set_raw_url(&mut self, raw_url: String) {
    self.raw_url = Some(raw_url);
  }

  pub fn with_raw_url(mut self, raw_url: String) -> ChangedFile {
    self.raw_url = Some(raw_url);
    self
  }

  pub fn raw_url(&self) -> Option<&String> {
    self.raw_url.as_ref()
  }

  pub fn reset_raw_url(&mut self) {
    self.raw_url = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> ChangedFile {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



