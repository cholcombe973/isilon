#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeDedupeSummary {
    ///
    #[serde(rename = "summary")]
    pub summary: Option <crate::models::DedupeDedupeSummarySummary>,
}
