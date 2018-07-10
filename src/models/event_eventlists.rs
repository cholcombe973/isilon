

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventlists {
  #[serde(rename = "eventlists")]
  eventlists: Option<Vec<::models::EventEventlist>>
}

