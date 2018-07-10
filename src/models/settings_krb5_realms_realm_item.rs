#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5RealmsRealmItem {
    /// Specifies the administrative server hostname.
    #[serde(rename = "admin_server")]
    pub admin_server: Option<String>,
    /// Specifies the default domain mapped to the realm.
    #[serde(rename = "default_domain")]
    pub default_domain: Option<String>,
    /// ID of realm
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// If true, indicates that the realm is the default.
    #[serde(rename = "is_default_realm")]
    pub is_default_realm: Option<bool>,
    /// If true, indicates that the realm is joined.
    #[serde(rename = "is_joined")]
    pub is_joined: Option<bool>,
    /// Specifies the list of KDC hostnames.
    #[serde(rename = "kdc")]
    pub kdc: Option<Vec<String>>,
    /// Specifies the name of the realm.
    #[serde(rename = "realm")]
    pub realm: Option<String>,
}
