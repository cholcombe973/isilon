/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventAlertConditionsExtended {
  #[serde(rename = "alert-conditions")]
  alert_conditions: Option<Vec<::models::EventAlertConditionsAlertCondition>>,
  /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

impl EventAlertConditionsExtended {
  pub fn new() -> EventAlertConditionsExtended {
    EventAlertConditionsExtended {
      alert_conditions: None,
      resume: None,
      total: None
    }
  }

  pub fn set_alert_conditions(&mut self, alert_conditions: Vec<::models::EventAlertConditionsAlertCondition>) {
    self.alert_conditions = Some(alert_conditions);
  }

  pub fn with_alert_conditions(mut self, alert_conditions: Vec<::models::EventAlertConditionsAlertCondition>) -> EventAlertConditionsExtended {
    self.alert_conditions = Some(alert_conditions);
    self
  }

  pub fn alert_conditions(&self) -> Option<&Vec<::models::EventAlertConditionsAlertCondition>> {
    self.alert_conditions.as_ref()
  }

  pub fn reset_alert_conditions(&mut self) {
    self.alert_conditions = None;
  }

  pub fn set_resume(&mut self, resume: String) {
    self.resume = Some(resume);
  }

  pub fn with_resume(mut self, resume: String) -> EventAlertConditionsExtended {
    self.resume = Some(resume);
    self
  }

  pub fn resume(&self) -> Option<&String> {
    self.resume.as_ref()
  }

  pub fn reset_resume(&mut self) {
    self.resume = None;
  }

  pub fn set_total(&mut self, total: i32) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: i32) -> EventAlertConditionsExtended {
    self.total = Some(total);
    self
  }

  pub fn total(&self) -> Option<&i32> {
    self.total.as_ref()
  }

  pub fn reset_total(&mut self) {
    self.total = None;
  }

}


