#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZonesSummary {
    /// The summary of a collection of objects.
    #[serde(rename = "summary")]
    pub summary: Option <crate::models::ZonesSummarySummary>,
}
