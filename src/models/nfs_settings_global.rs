

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsGlobal {
  /// Specifies the global NFS configuration settings.
  #[serde(rename = "settings")]
  settings: Option<::models::NfsSettingsGlobalSettings>
}

