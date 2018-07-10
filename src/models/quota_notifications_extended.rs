

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNotificationsExtended {
  #[serde(rename = "notifications")]
  notifications: Option<Vec<::models::QuotaNotificationExtended>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

