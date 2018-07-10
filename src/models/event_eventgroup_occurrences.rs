

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupOccurrences {
  #[serde(rename = "eventgroups")]
  eventgroups: Option<Vec<::models::EventEventgroupOccurrencesEventgroup>>
}

