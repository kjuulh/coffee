/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// WikiPage : WikiPage a wiki page

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiPage {
  #[serde(rename = "commit_count")]
  commit_count: Option<i64>,
  /// Page content, base64 encoded
  #[serde(rename = "content_base64")]
  content_base64: Option<String>,
  #[serde(rename = "footer")]
  footer: Option<String>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "last_commit")]
  last_commit: Option<::models::WikiCommit>,
  #[serde(rename = "sidebar")]
  sidebar: Option<String>,
  #[serde(rename = "sub_url")]
  sub_url: Option<String>,
  #[serde(rename = "title")]
  title: Option<String>
}

impl WikiPage {
  /// WikiPage a wiki page
  pub fn new() -> WikiPage {
    WikiPage {
      commit_count: None,
      content_base64: None,
      footer: None,
      html_url: None,
      last_commit: None,
      sidebar: None,
      sub_url: None,
      title: None
    }
  }

  pub fn set_commit_count(&mut self, commit_count: i64) {
    self.commit_count = Some(commit_count);
  }

  pub fn with_commit_count(mut self, commit_count: i64) -> WikiPage {
    self.commit_count = Some(commit_count);
    self
  }

  pub fn commit_count(&self) -> Option<&i64> {
    self.commit_count.as_ref()
  }

  pub fn reset_commit_count(&mut self) {
    self.commit_count = None;
  }

  pub fn set_content_base64(&mut self, content_base64: String) {
    self.content_base64 = Some(content_base64);
  }

  pub fn with_content_base64(mut self, content_base64: String) -> WikiPage {
    self.content_base64 = Some(content_base64);
    self
  }

  pub fn content_base64(&self) -> Option<&String> {
    self.content_base64.as_ref()
  }

  pub fn reset_content_base64(&mut self) {
    self.content_base64 = None;
  }

  pub fn set_footer(&mut self, footer: String) {
    self.footer = Some(footer);
  }

  pub fn with_footer(mut self, footer: String) -> WikiPage {
    self.footer = Some(footer);
    self
  }

  pub fn footer(&self) -> Option<&String> {
    self.footer.as_ref()
  }

  pub fn reset_footer(&mut self) {
    self.footer = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> WikiPage {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_last_commit(&mut self, last_commit: ::models::WikiCommit) {
    self.last_commit = Some(last_commit);
  }

  pub fn with_last_commit(mut self, last_commit: ::models::WikiCommit) -> WikiPage {
    self.last_commit = Some(last_commit);
    self
  }

  pub fn last_commit(&self) -> Option<&::models::WikiCommit> {
    self.last_commit.as_ref()
  }

  pub fn reset_last_commit(&mut self) {
    self.last_commit = None;
  }

  pub fn set_sidebar(&mut self, sidebar: String) {
    self.sidebar = Some(sidebar);
  }

  pub fn with_sidebar(mut self, sidebar: String) -> WikiPage {
    self.sidebar = Some(sidebar);
    self
  }

  pub fn sidebar(&self) -> Option<&String> {
    self.sidebar.as_ref()
  }

  pub fn reset_sidebar(&mut self) {
    self.sidebar = None;
  }

  pub fn set_sub_url(&mut self, sub_url: String) {
    self.sub_url = Some(sub_url);
  }

  pub fn with_sub_url(mut self, sub_url: String) -> WikiPage {
    self.sub_url = Some(sub_url);
    self
  }

  pub fn sub_url(&self) -> Option<&String> {
    self.sub_url.as_ref()
  }

  pub fn reset_sub_url(&mut self) {
    self.sub_url = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> WikiPage {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

}



