#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventAlertConditionCreateParams {
    /// Event Group categories to be alerted: all, 100000000 (SYS_DISK_EVENTS), 200000000 (NODE_STATUS_EVENTS), 300000000 (REBOOT_EVENTS), 400000000 (SW_EVENTS), 500000000 (QUOTA_EVENTS), 600000000 (SNAP_EVENTS), 700000000 (WINNET_EVENTS), 800000000 (FILESYS_EVENTS), 900000000 (HW_EVENTS), 1100000000 (CPOOL_EVENTS)
    #[serde(rename = "categories")]
    pub categories: Option<Vec<String>>,
    /// Channels for alert
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    /// Trigger condition for alert
    #[serde(rename = "condition")]
    pub condition: String,
    /// Event Group IDs to be alerted
    #[serde(rename = "eventgroup_ids")]
    pub eventgroup_ids: Option<Vec<String>>,
    /// Unique identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Required with ONGOING condition only, period in seconds between alerts of ongoing conditions
    #[serde(rename = "interval")]
    pub interval: Option<i32>,
    /// Required with NEW EVENTS condition only, limits the number of alerts sent as events are added
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// Unique identifier.
    #[serde(rename = "name")]
    pub name: String,
    /// Severities to be alerted
    #[serde(rename = "severities")]
    pub severities: Option<Vec<String>>,
    /// Any eventgroup lasting less than this many seconds is deemed transient and will not generate alerts under this condition.
    #[serde(rename = "transient")]
    pub transient: Option<i32>,
}
