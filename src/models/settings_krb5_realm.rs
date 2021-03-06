/// SettingsKrb5Realm : Specifies the Kerberos settings for realm access.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5Realm {
    /// Specifies the administrative server hostname.
    #[serde(rename = "admin_server")]
    pub admin_server: Option<String>,
    /// Specifies the default domain mapped to the realm.
    #[serde(rename = "default_domain")]
    pub default_domain: Option<String>,
    /// If true, indicates that the realm is the default.
    #[serde(rename = "is_default_realm")]
    pub is_default_realm: Option<bool>,
    /// Specifies the list of KDC hostnames.
    #[serde(rename = "kdc")]
    pub kdc: Option<Vec<String>>,
}
