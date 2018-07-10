

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsDmas {
  #[serde(rename = "dmavendors")]
  dmavendors: Option<Vec<::models::NdmpSettingsDmasDmavendor>>
}

