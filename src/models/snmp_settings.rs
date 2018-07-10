

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnmpSettings {
  /// SNMP settings.
  #[serde(rename = "settings")]
  settings: Option<::models::SnmpSettingsSettings>
}

