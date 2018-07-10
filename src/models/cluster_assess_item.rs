/// ClusterAssessItem : The settings necessary to start a pre-upgrade assessment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterAssessItem {
    /// The location (path) of the upgrade image which must be within /ifs.
    #[serde(rename = "install_image_path")]
    pub install_image_path: Option<String>,
}
