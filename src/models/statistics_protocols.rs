

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsProtocols {
  #[serde(rename = "protocols")]
  protocols: Vec<::models::StatisticsProtocol>
}

