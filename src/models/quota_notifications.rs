#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNotifications {
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<::models::QuotaNotificationExtended>>,
}
