
/// EventEventgroupOccurrence : Ignore flag for eventgroup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupOccurrence {
  /// True if eventgroup is to be ignored
  #[serde(rename = "ignore")]
  ignore: Option<bool>,
  /// True if eventgroup is to be resolved
  #[serde(rename = "resolved")]
  resolved: Option<bool>
}

