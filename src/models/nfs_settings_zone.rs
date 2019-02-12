#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsZone {
    /// Specifies the per-zone NFS configuration settings.
    #[serde(rename = "settings")]
    pub settings: Option <crate::models::NfsSettingsZoneSettings>,
}
