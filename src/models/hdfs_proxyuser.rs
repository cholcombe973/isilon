

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsProxyuser {
  /// The ID of the role.
  #[serde(rename = "id")]
  id: String,
  /// Users or groups impersonated by proxyuser.
  #[serde(rename = "members")]
  members: Vec<::models::AuthAccessAccessItemFileGroup>,
  /// The name of the proxyuser.
  #[serde(rename = "name")]
  name: String
}

