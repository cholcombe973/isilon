

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsGlobal {
  /// 
  #[serde(rename = "global")]
  global: Option<::models::NdmpSettingsGlobalGlobal>
}

