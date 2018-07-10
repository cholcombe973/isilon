#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSettingsGlobalSettingsAuditGlobalSaclItem {
    /// Determines if audit is performed on success or failure.
    #[serde(rename = "flags")]
    pub flags: String,
    /// Specifies the array of filesystem rights that are governed.
    #[serde(rename = "permission")]
    pub permission: Vec<String>,
}
