#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolStats {
    ///
    #[serde(rename = "protocol-stats")]
    pub protocol_stats: Option <crate::models::SummaryProtocolStatsProtocolStats>,
}
