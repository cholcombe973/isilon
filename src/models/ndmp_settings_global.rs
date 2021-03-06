#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsGlobal {
    ///
    #[serde(rename = "global")]
    pub global: Option <crate::models::NdmpSettingsGlobalGlobal>,
}
