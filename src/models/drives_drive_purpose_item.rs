
/// DrivesDrivePurposeItem : Drive purpose information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDrivePurposeItem {
  /// The purpose to which this drive should be assigned. This field is required for the 'purpose' action.
  #[serde(rename = "purpose")]
  purpose: String
}

