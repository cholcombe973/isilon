

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdsProviderControllersController {
  /// Specifies the address for the domain controller.
  #[serde(rename = "dc_address")]
  dc_address: Option<String>,
  /// Specifies the name of the domain controller.
  #[serde(rename = "dc_name")]
  dc_name: Option<String>,
  /// Specifies the address for the domain controller. This value is the same as the 'dc_address' value.
  #[serde(rename = "id")]
  id: Option<String>
}

