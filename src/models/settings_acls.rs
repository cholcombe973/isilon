

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAcls {
  /// ACL policies settings.
  #[serde(rename = "acl_policy_settings")]
  acl_policy_settings: Option<::models::SettingsAclsAclPolicySettings>
}

