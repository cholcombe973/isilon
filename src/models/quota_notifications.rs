

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNotifications {
  #[serde(rename = "notifications")]
  notifications: Option<Vec<::models::QuotaNotificationExtended>>
}

