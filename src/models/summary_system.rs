#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarySystem {
    #[serde(rename = "system")]
    pub system: Option<Vec<::models::SummarySystemSystemItem>>,
}
