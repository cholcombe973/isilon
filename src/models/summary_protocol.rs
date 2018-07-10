

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocol {
  #[serde(rename = "protocol")]
  protocol: Option<Vec<::models::SummaryProtocolProtocolItem>>
}

