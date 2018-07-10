

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStats {
  /// 
  #[serde(rename = "cpu")]
  cpu: Option<::models::SummaryProtocolStatsProtocolStatsCpu>,
  /// 
  #[serde(rename = "disk")]
  disk: Option<::models::SummaryProtocolStatsProtocolStatsDisk>,
  /// 
  #[serde(rename = "network")]
  network: Option<::models::SummaryProtocolStatsProtocolStatsNetwork>,
  /// 
  #[serde(rename = "onefs")]
  onefs: Option<::models::SummaryProtocolStatsProtocolStatsOnefs>,
  /// A single protocol for which statistics should be reported.
  #[serde(rename = "protocol")]
  protocol: Option<::models::SummaryProtocolStatsProtocolStatsProtocol>,
  /// Unix Epoch time in seconds of the request.
  #[serde(rename = "time")]
  time: i32
}

