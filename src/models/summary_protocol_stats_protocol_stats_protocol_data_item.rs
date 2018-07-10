

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsProtocolDataItem {
  /// The name of the protocol operation.
  #[serde(rename = "name")]
  name: String,
  /// Protocol specific operations per second. This is a variable number of output fields depending on the protocol being displayed.
  #[serde(rename = "value")]
  value: String
}

