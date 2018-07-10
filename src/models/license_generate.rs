

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseGenerate {
  /// Array of licenses included in activation file.
  #[serde(rename = "activated_license_list")]
  activated_license_list: Option<Vec<String>>,
  /// Contents of licensing activation file.
  #[serde(rename = "activation")]
  activation: Option<String>,
  /// Array of licenses included in activation file.
  #[serde(rename = "hardware")]
  hardware: Option<Vec<::models::LicenseGenerateHardwareItem>>,
  /// An array of licenses not included in activation file.
  #[serde(rename = "not_activated_license_list")]
  not_activated_license_list: Option<Vec<String>>
}

