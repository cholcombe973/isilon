/// ClusterPatchPatch : A software patch that can be applied to OneFS.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterPatchPatch {
    /// The path location of the patch file.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// The id or filename of the patch to install.
    #[serde(rename = "patch")]
    pub patch: String,
}
