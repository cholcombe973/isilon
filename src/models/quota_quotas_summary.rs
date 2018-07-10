

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotasSummary {
  /// 
  #[serde(rename = "summary")]
  summary: Option<::models::QuotaQuotasSummarySummary>
}

