

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditTopics {
  #[serde(rename = "topics")]
  topics: Option<Vec<::models::AuditTopicExtended>>
}

