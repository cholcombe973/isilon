

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNetgroup {
  /// NFS netgroup cache settings.
  #[serde(rename = "settings")]
  settings: Option<::models::NfsNetgroupSettings>
}

