#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAcls {
    /// ACL policies settings.
    #[serde(rename = "acl_policy_settings")]
    pub acl_policy_settings: Option <crate::models::SettingsAclsAclPolicySettings>,
}
