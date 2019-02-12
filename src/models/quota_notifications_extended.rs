#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNotificationsExtended {
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec <crate::models::QuotaNotificationExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
