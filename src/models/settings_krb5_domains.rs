#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5Domains {
    #[serde(rename = "domain")]
    pub domain: Option<Vec <crate::models::SettingsKrb5DomainsDomainItem>>,
}
