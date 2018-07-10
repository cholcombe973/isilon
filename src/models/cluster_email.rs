#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterEmail {
    /// Cluster email notification settings.
    #[serde(rename = "settings")]
    pub settings: Option<::models::ClusterEmailSettings>,
}
