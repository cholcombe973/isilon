/// NfsSettingsExport : Default NFS export configuration values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsExport {
    /// Specifies configuration values for NFS exports.
    #[serde(rename = "settings")]
    pub settings: Option<::models::NfsSettingsExportSettings>,
}
