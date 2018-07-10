

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5RealmCreateParams {
  /// Specifies the administrative server hostname.
  #[serde(rename = "admin_server")]
  admin_server: Option<String>,
  /// Specifies the default domain mapped to the realm.
  #[serde(rename = "default_domain")]
  default_domain: Option<String>,
  /// If true, indicates that the realm is the default.
  #[serde(rename = "is_default_realm")]
  is_default_realm: Option<bool>,
  /// Specifies the list of KDC hostnames.
  #[serde(rename = "kdc")]
  kdc: Option<Vec<String>>,
  /// Specifies the name of the realm.
  #[serde(rename = "realm")]
  realm: String
}

