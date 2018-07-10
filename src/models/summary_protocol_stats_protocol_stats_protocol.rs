#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsProtocol {
    #[serde(rename = "data")]
    pub data: Vec<::models::SummaryProtocolStatsProtocolStatsProtocolDataItem>,
    /// The name of the protocol.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
