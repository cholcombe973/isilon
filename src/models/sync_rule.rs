/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SyncRule : A rule limiting the bandwidth, file operations, cpu usage, or worker count for all sync jobs on this cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRule {
  /// User-entered description of this performance rule.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Whether this performance rule is currently in effect during its specified intervals.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Amount the specified system resource type is limited by this rule.  Units are kb/s for bandwidth, files/s for file-count, processing percentage used for cpu, or percentage of maximum available workers.
  #[serde(rename = "limit")]
  limit: Option<i32>,
  /// A schedule defining when during a week this performance rule is in effect.  If unspecified or null, the schedule will always be in effect.
  #[serde(rename = "schedule")]
  schedule: Option<::models::SyncRuleSchedule>
}

impl SyncRule {
  /// A rule limiting the bandwidth, file operations, cpu usage, or worker count for all sync jobs on this cluster.
  pub fn new() -> SyncRule {
    SyncRule {
      description: None,
      enabled: None,
      limit: None,
      schedule: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> SyncRule {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> SyncRule {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_limit(&mut self, limit: i32) {
    self.limit = Some(limit);
  }

  pub fn with_limit(mut self, limit: i32) -> SyncRule {
    self.limit = Some(limit);
    self
  }

  pub fn limit(&self) -> Option<&i32> {
    self.limit.as_ref()
  }

  pub fn reset_limit(&mut self) {
    self.limit = None;
  }

  pub fn set_schedule(&mut self, schedule: ::models::SyncRuleSchedule) {
    self.schedule = Some(schedule);
  }

  pub fn with_schedule(mut self, schedule: ::models::SyncRuleSchedule) -> SyncRule {
    self.schedule = Some(schedule);
    self
  }

  pub fn schedule(&self) -> Option<&::models::SyncRuleSchedule> {
    self.schedule.as_ref()
  }

  pub fn reset_schedule(&mut self) {
    self.schedule = None;
  }

}


