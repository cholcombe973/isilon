#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsNetworkOut {
    /// Network output errors per-second
    #[serde(rename = "errors_per_sec")]
    pub errors_per_sec: f32,
    /// Network output megabytes per-second
    #[serde(rename = "megabytes_per_sec")]
    pub megabytes_per_sec: f32,
    /// Network output packets per-second
    #[serde(rename = "packets_per_sec")]
    pub packets_per_sec: f32,
}
