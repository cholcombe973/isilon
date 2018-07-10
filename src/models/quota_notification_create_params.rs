#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNotificationCreateParams {
    /// Send alert when rule matches.
    #[serde(rename = "action_alert")]
    pub action_alert: Option<bool>,
    /// Email a specific email address when rule matches.
    #[serde(rename = "action_email_address")]
    pub action_email_address: Option<String>,
    /// Email quota domain owner when rule matches.
    #[serde(rename = "action_email_owner")]
    pub action_email_owner: Option<bool>,
    /// Path of optional /ifs template file used for email actions.
    #[serde(rename = "email_template")]
    pub email_template: Option<String>,
    /// Time to wait between detections for rules triggered by user actions.
    #[serde(rename = "holdoff")]
    pub holdoff: Option<i32>,
    /// Schedule for rules that repeatedly notify.
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
    /// The condition detected.
    #[serde(rename = "condition")]
    pub condition: String,
    /// The quota threshold detected.
    #[serde(rename = "threshold")]
    pub threshold: String,
}
