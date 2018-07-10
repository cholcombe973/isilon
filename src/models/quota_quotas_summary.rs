#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotasSummary {
    ///
    #[serde(rename = "summary")]
    pub summary: Option<::models::QuotaQuotasSummarySummary>,
}
