
/// WormSettingsExtended : Specifies global SmartLock (WORM) settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormSettingsExtended {
  /// To set the compliance clock to the current system time, PUT to this resource with an empty JSON object {} for the cdate value.  This cluster must be in compliance mode to set the compliance clock.
  #[serde(rename = "cdate")]
  cdate: Option<::models::Empty>
}

