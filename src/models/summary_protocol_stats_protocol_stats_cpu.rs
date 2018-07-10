#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsCpu {
    /// Percentage of CPU idle time
    #[serde(rename = "idle")]
    pub idle: f32,
    /// Percentage of CPU consumed by the system
    #[serde(rename = "system")]
    pub system: f32,
    /// Percentage of CPU consumed by user activities
    #[serde(rename = "user")]
    pub user: f32,
}
