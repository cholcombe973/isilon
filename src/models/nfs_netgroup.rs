#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNetgroup {
    /// NFS netgroup cache settings.
    #[serde(rename = "settings")]
    pub settings: Option <crate::models::NfsNetgroupSettings>,
}
