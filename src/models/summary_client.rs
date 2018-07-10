

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryClient {
  #[serde(rename = "client")]
  client: Option<Vec<::models::SummaryClientClientItem>>
}

