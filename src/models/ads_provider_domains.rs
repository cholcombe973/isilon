

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdsProviderDomains {
  #[serde(rename = "domains")]
  domains: Option<Vec<::models::AdsProviderDomainsDomain>>
}

