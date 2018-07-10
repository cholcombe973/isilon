

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNotificationCreateParams {
  /// Send alert when rule matches.
  #[serde(rename = "action_alert")]
  action_alert: Option<bool>,
  /// Email a specific email address when rule matches.
  #[serde(rename = "action_email_address")]
  action_email_address: Option<String>,
  /// Email quota domain owner when rule matches.
  #[serde(rename = "action_email_owner")]
  action_email_owner: Option<bool>,
  /// Path of optional /ifs template file used for email actions.
  #[serde(rename = "email_template")]
  email_template: Option<String>,
  /// Time to wait between detections for rules triggered by user actions.
  #[serde(rename = "holdoff")]
  holdoff: Option<i32>,
  /// Schedule for rules that repeatedly notify.
  #[serde(rename = "schedule")]
  schedule: Option<String>,
  /// The condition detected.
  #[serde(rename = "condition")]
  condition: String,
  /// The quota threshold detected.
  #[serde(rename = "threshold")]
  threshold: String
}

