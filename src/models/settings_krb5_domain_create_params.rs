

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5DomainCreateParams {
  /// Specifies the name of the realm.
  #[serde(rename = "realm")]
  realm: String,
  /// Specifies the name of the domain.
  #[serde(rename = "domain")]
  domain: String
}

