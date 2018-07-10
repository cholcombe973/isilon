
/// CloudAccessItem : Accessible cloud storage information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudAccessItem {
  /// A cluster guid indicating the birth place of one or more accounts or policies on this cluster
  #[serde(rename = "guid")]
  guid: String
}

