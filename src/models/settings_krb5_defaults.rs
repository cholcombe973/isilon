

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5Defaults {
  /// Specifies the properties for the global Kerberos authentication settings.
  #[serde(rename = "krb5_settings")]
  krb5_settings: Option<::models::SettingsKrb5DefaultsKrb5Settings>
}

