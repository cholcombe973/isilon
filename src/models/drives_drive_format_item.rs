
/// DrivesDriveFormatItem : Drive purpose information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFormatItem {
  /// The purpose to which this drive should be formatted. If not specified, defaults to 'None', which will be automatically purposed based on node configuration and drive type.
  #[serde(rename = "purpose")]
  purpose: Option<String>
}

