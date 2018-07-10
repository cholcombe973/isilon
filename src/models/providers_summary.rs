#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersSummary {
    #[serde(rename = "provider_instances")]
    pub provider_instances: Option<Vec<::models::ProvidersSummaryProviderInstance>>,
}
