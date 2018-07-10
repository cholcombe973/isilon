

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterPatchPatches {
  #[serde(rename = "patches")]
  patches: Option<Vec<::models::ClusterPatchPatchesPatch>>
}

