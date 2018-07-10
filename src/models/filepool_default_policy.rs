
/// FilepoolDefaultPolicy : A default filepool policy object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolDefaultPolicy {
  /// A default filepool policy object
  #[serde(rename = "default-policy")]
  default_policy: Option<::models::FilepoolDefaultPolicyDefaultPolicy>
}

