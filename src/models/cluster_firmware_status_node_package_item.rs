#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFirmwareStatusNodePackageItem {
    /// Valid checksum string for binary. One of the following: 'ok', 'bad', 'file_missing', or 'na'
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,
    /// The name of the firmware binary.
    #[serde(rename = "firmware")]
    pub firmware: Option<String>,
    /// The version string for the binary.
    #[serde(rename = "version")]
    pub version: Option<String>,
}
