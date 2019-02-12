#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStatsProtocolStats {
    ///
    #[serde(rename = "cpu")]
    pub cpu: Option <crate::models::SummaryProtocolStatsProtocolStatsCpu>,
    ///
    #[serde(rename = "disk")]
    pub disk: Option <crate::models::SummaryProtocolStatsProtocolStatsDisk>,
    ///
    #[serde(rename = "network")]
    pub network: Option <crate::models::SummaryProtocolStatsProtocolStatsNetwork>,
    ///
    #[serde(rename = "onefs")]
    pub onefs: Option <crate::models::SummaryProtocolStatsProtocolStatsOnefs>,
    /// A single protocol for which statistics should be reported.
    #[serde(rename = "protocol")]
    pub protocol: Option <crate::models::SummaryProtocolStatsProtocolStatsProtocol>,
    /// Unix Epoch time in seconds of the request.
    #[serde(rename = "time")]
    pub time: i32,
}
