/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.20.0+rc0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Cron : Cron represents a Cron task

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cron {
  #[serde(rename = "exec_times")]
  exec_times: Option<i64>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "next")]
  next: Option<String>,
  #[serde(rename = "prev")]
  prev: Option<String>,
  #[serde(rename = "schedule")]
  schedule: Option<String>
}

impl Cron {
  /// Cron represents a Cron task
  pub fn new() -> Cron {
    Cron {
      exec_times: None,
      name: None,
      next: None,
      prev: None,
      schedule: None
    }
  }

  pub fn set_exec_times(&mut self, exec_times: i64) {
    self.exec_times = Some(exec_times);
  }

  pub fn with_exec_times(mut self, exec_times: i64) -> Cron {
    self.exec_times = Some(exec_times);
    self
  }

  pub fn exec_times(&self) -> Option<&i64> {
    self.exec_times.as_ref()
  }

  pub fn reset_exec_times(&mut self) {
    self.exec_times = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Cron {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_next(&mut self, next: String) {
    self.next = Some(next);
  }

  pub fn with_next(mut self, next: String) -> Cron {
    self.next = Some(next);
    self
  }

  pub fn next(&self) -> Option<&String> {
    self.next.as_ref()
  }

  pub fn reset_next(&mut self) {
    self.next = None;
  }

  pub fn set_prev(&mut self, prev: String) {
    self.prev = Some(prev);
  }

  pub fn with_prev(mut self, prev: String) -> Cron {
    self.prev = Some(prev);
    self
  }

  pub fn prev(&self) -> Option<&String> {
    self.prev.as_ref()
  }

  pub fn reset_prev(&mut self) {
    self.prev = None;
  }

  pub fn set_schedule(&mut self, schedule: String) {
    self.schedule = Some(schedule);
  }

  pub fn with_schedule(mut self, schedule: String) -> Cron {
    self.schedule = Some(schedule);
    self
  }

  pub fn schedule(&self) -> Option<&String> {
    self.schedule.as_ref()
  }

  pub fn reset_schedule(&mut self) {
    self.schedule = None;
  }

}



