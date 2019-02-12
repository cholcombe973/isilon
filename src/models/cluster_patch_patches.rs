#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterPatchPatches {
    #[serde(rename = "patches")]
    pub patches: Option<Vec <crate::models::ClusterPatchPatchesPatch>>,
}
