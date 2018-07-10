#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5DefaultsKrb5Settings {
    /// If true, allow the use of DES encryption
    #[serde(rename = "allow_weak_crypto")]
    pub allow_weak_crypto: Option<bool>,
    /// If true, always attempts to preauthenticate to the domain controller.
    #[serde(rename = "always_send_preauth")]
    pub always_send_preauth: Option<bool>,
    /// Specifies the realm for unqualified names.
    #[serde(rename = "default_realm")]
    pub default_realm: Option<String>,
    /// If true, find KDCs through the DNS.
    #[serde(rename = "dns_lookup_kdc")]
    pub dns_lookup_kdc: Option<bool>,
    /// If true, find realm names through the DNS.
    #[serde(rename = "dns_lookup_realm")]
    pub dns_lookup_realm: Option<bool>,
}
