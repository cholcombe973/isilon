#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLdapTemplatesLdapFieldTemplateItem {
    /// Specifies canonical name.
    #[serde(rename = "cn_attribute")]
    pub cn_attribute: Option<String>,
    /// Sets hashed password value.
    #[serde(rename = "crypt_password_attribute")]
    pub crypt_password_attribute: Option<String>,
    /// Sets the LDAP Email attribute.
    #[serde(rename = "email_attribute")]
    pub email_attribute: Option<String>,
    /// Sets the LDAP GECOS attribute.
    #[serde(rename = "gecos_attribute")]
    pub gecos_attribute: Option<String>,
    /// Sets the LDAP GID attribute.
    #[serde(rename = "gid_attribute")]
    pub gid_attribute: Option<String>,
    /// Sets LDAP filter for group objects.
    #[serde(rename = "group_filter")]
    pub group_filter: Option<String>,
    /// Sets the LDAP Group Members attribute.
    #[serde(rename = "group_members_attribute")]
    pub group_members_attribute: Option<String>,
    /// Sets the LDAP Homedir attribute.
    #[serde(rename = "homedir_attribute")]
    pub homedir_attribute: Option<String>,
    /// Specifies the ID of the LDAP provider field template.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Sets the LDAP UID attribute, which is used as the login name.
    #[serde(rename = "name_attribute")]
    pub name_attribute: Option<String>,
    /// Sets LDAP filter for netgroup objects.
    #[serde(rename = "netgroup_filter")]
    pub netgroup_filter: Option<String>,
    /// Sets the LDAP Netgroup Members attribute.
    #[serde(rename = "netgroup_members_attribute")]
    pub netgroup_members_attribute: Option<String>,
    /// Sets the LDAP Netgroup Triple attribute.
    #[serde(rename = "netgroup_triple_attribute")]
    pub netgroup_triple_attribute: Option<String>,
    /// Sets the LDAP NT Password attribute.
    #[serde(rename = "nt_password_attribute")]
    pub nt_password_attribute: Option<String>,
    /// Sets the absolute date to expire the account.
    #[serde(rename = "shadow_expire_attribute")]
    pub shadow_expire_attribute: Option<String>,
    /// Sets the section of the shadow map that is used to store the flag value.
    #[serde(rename = "shadow_flag_attribute")]
    pub shadow_flag_attribute: Option<String>,
    /// Sets the number of days of inactivity that is allowed for the user.
    #[serde(rename = "shadow_inactive_attribute")]
    pub shadow_inactive_attribute: Option<String>,
    /// Sets the last change of the shadow information.
    #[serde(rename = "shadow_last_change_attribute")]
    pub shadow_last_change_attribute: Option<String>,
    /// Sets the maximum number of days a password can be valid.
    #[serde(rename = "shadow_max_attribute")]
    pub shadow_max_attribute: Option<String>,
    /// Sets the minimum number of days between shadow changes.
    #[serde(rename = "shadow_min_attribute")]
    pub shadow_min_attribute: Option<String>,
    /// Sets LDAP filter for shadow user objects.
    #[serde(rename = "shadow_user_filter")]
    pub shadow_user_filter: Option<String>,
    /// Sets the number of days before the password expires to warn the user.
    #[serde(rename = "shadow_warning_attribute")]
    pub shadow_warning_attribute: Option<String>,
    /// Sets the LDAP Shell attribute.
    #[serde(rename = "shell_attribute")]
    pub shell_attribute: Option<String>,
    /// Sets the LDAP UID Number attribute.
    #[serde(rename = "uid_attribute")]
    pub uid_attribute: Option<String>,
    /// Sets the LDAP Unique Group Members attribute.
    #[serde(rename = "unique_group_members_attribute")]
    pub unique_group_members_attribute: Option<String>,
    /// Sets LDAP filter for user objects.
    #[serde(rename = "user_filter")]
    pub user_filter: Option<String>,
}
