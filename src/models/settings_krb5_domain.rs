/// SettingsKrb5Domain : Specifies the Kerberos settings for domain-realm mappings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5Domain {
    /// Specifies the name of the realm.
    #[serde(rename = "realm")]
    pub realm: Option<String>,
}
