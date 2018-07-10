#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5DomainsDomainItem {
    /// Specifies the name of the domain.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// ID of the domain
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the name of the realm.
    #[serde(rename = "realm")]
    pub realm: Option<String>,
}
