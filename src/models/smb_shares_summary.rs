#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSharesSummary {
    ///
    #[serde(rename = "summary")]
    pub summary: Option<::models::SmbSharesSummarySummary>,
}
