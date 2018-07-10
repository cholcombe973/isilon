#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterPatchPatchesPatch {
    /// A long comment about the patch.
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// Other patches that this patch conflicts with.
    #[serde(rename = "conflicts")]
    pub conflicts: Option<Vec<String>>,
    /// Other patches that this patch depends on.
    #[serde(rename = "dependencies")]
    pub dependencies: Option<Vec<String>>,
    /// A short description of the patch.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The files contained in this patch.
    #[serde(rename = "files")]
    pub files: Option<Vec<::models::ClusterPatchPatchesPatchFile>>,
    /// A unique identifier for the patch.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the patch.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The nodes that this patch is installed on.
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<i32>>,
    /// Describes the reboot requirements
    #[serde(rename = "reboot")]
    pub reboot: Option<String>,
    /// The services affected during the patch deployment
    #[serde(rename = "services")]
    pub services: Option<Vec<::models::ClusterPatchPatchesPatchService>>,
    /// The intallation status of this patch on the cluster.
    #[serde(rename = "status")]
    pub status: Option<String>,
}
