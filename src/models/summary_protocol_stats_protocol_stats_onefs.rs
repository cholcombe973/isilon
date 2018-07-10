

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsOnefs {
  /// OneFS throughput in MB/s in.
  #[serde(rename = "in")]
  _in: f32,
  /// OneFS throughput in MB/s out.
  #[serde(rename = "out")]
  out: f32,
  /// OneFS throughput in MB/s total.
  #[serde(rename = "total")]
  total: f32
}

