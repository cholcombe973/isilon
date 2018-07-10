

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5Domains {
  #[serde(rename = "domain")]
  domain: Option<Vec<::models::SettingsKrb5DomainsDomainItem>>
}

