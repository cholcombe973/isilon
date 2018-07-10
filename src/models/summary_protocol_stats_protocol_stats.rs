#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStats {
    ///
    #[serde(rename = "cpu")]
    pub cpu: Option<::models::SummaryProtocolStatsProtocolStatsCpu>,
    ///
    #[serde(rename = "disk")]
    pub disk: Option<::models::SummaryProtocolStatsProtocolStatsDisk>,
    ///
    #[serde(rename = "network")]
    pub network: Option<::models::SummaryProtocolStatsProtocolStatsNetwork>,
    ///
    #[serde(rename = "onefs")]
    pub onefs: Option<::models::SummaryProtocolStatsProtocolStatsOnefs>,
    /// A single protocol for which statistics should be reported.
    #[serde(rename = "protocol")]
    pub protocol: Option<::models::SummaryProtocolStatsProtocolStatsProtocol>,
    /// Unix Epoch time in seconds of the request.
    #[serde(rename = "time")]
    pub time: i32,
}
