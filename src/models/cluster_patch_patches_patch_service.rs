#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterPatchPatchesPatchService {
    /// Description of the affected service
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the affected service
    #[serde(rename = "name")]
    pub name: Option<String>,
}
