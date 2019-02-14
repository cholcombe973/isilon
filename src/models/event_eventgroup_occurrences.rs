#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupOccurrences {
    #[serde(rename = "eventgroups")]
    pub eventgroups: Option<Vec <crate::models::EventEventgroupOccurrencesEventgroup>>,
}
