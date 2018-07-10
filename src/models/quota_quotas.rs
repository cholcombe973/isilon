#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotas {
    #[serde(rename = "quotas")]
    pub quotas: Option<Vec<::models::QuotaQuotaExtended>>,
}
