

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsNetworkIn {
  /// Network input errors per-second
  #[serde(rename = "errors_per_sec")]
  errors_per_sec: f32,
  /// Network input megabytes per-second
  #[serde(rename = "megabytes_per_sec")]
  megabytes_per_sec: f32,
  /// Network input packets per-second
  #[serde(rename = "packets_per_sec")]
  packets_per_sec: f32
}

