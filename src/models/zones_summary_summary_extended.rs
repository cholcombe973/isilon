#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZonesSummarySummaryExtended {
    /// The count of objects in the collection
    #[serde(rename = "count")]
    pub count: i32,
    /// List of zone names
    #[serde(rename = "list")]
    pub list: Option<Vec<String>>,
}
