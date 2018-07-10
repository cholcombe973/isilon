

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsCpu {
  /// Percentage of CPU idle time
  #[serde(rename = "idle")]
  idle: f32,
  /// Percentage of CPU consumed by the system
  #[serde(rename = "system")]
  system: f32,
  /// Percentage of CPU consumed by user activities
  #[serde(rename = "user")]
  user: f32
}

