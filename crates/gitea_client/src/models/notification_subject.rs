/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NotificationSubject : NotificationSubject contains the notification subject (Issue/Pull/Commit)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSubject {
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "latest_comment_html_url")]
  latest_comment_html_url: Option<String>,
  #[serde(rename = "latest_comment_url")]
  latest_comment_url: Option<String>,
  #[serde(rename = "state")]
  state: Option<::models::StateType>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "type")]
  _type: Option<::models::NotifySubjectType>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl NotificationSubject {
  /// NotificationSubject contains the notification subject (Issue/Pull/Commit)
  pub fn new() -> NotificationSubject {
    NotificationSubject {
      html_url: None,
      latest_comment_html_url: None,
      latest_comment_url: None,
      state: None,
      title: None,
      _type: None,
      url: None
    }
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> NotificationSubject {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_latest_comment_html_url(&mut self, latest_comment_html_url: String) {
    self.latest_comment_html_url = Some(latest_comment_html_url);
  }

  pub fn with_latest_comment_html_url(mut self, latest_comment_html_url: String) -> NotificationSubject {
    self.latest_comment_html_url = Some(latest_comment_html_url);
    self
  }

  pub fn latest_comment_html_url(&self) -> Option<&String> {
    self.latest_comment_html_url.as_ref()
  }

  pub fn reset_latest_comment_html_url(&mut self) {
    self.latest_comment_html_url = None;
  }

  pub fn set_latest_comment_url(&mut self, latest_comment_url: String) {
    self.latest_comment_url = Some(latest_comment_url);
  }

  pub fn with_latest_comment_url(mut self, latest_comment_url: String) -> NotificationSubject {
    self.latest_comment_url = Some(latest_comment_url);
    self
  }

  pub fn latest_comment_url(&self) -> Option<&String> {
    self.latest_comment_url.as_ref()
  }

  pub fn reset_latest_comment_url(&mut self) {
    self.latest_comment_url = None;
  }

  pub fn set_state(&mut self, state: ::models::StateType) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::StateType) -> NotificationSubject {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::StateType> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> NotificationSubject {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set__type(&mut self, _type: ::models::NotifySubjectType) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: ::models::NotifySubjectType) -> NotificationSubject {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&::models::NotifySubjectType> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> NotificationSubject {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



