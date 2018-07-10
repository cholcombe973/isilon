#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStatsNetwork {
    ///
    #[serde(rename = "in")]
    pub _in: Option<::models::SummaryProtocolStatsProtocolStatsNetworkIn>,
    ///
    #[serde(rename = "out")]
    pub out: Option<::models::SummaryProtocolStatsProtocolStatsNetworkOut>,
}
