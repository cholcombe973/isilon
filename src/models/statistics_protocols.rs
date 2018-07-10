#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsProtocols {
    #[serde(rename = "protocols")]
    pub protocols: Vec<::models::StatisticsProtocol>,
}
