#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLdapTemplatesExtended {
    #[serde(rename = "ldap_configuration_templates")]
    pub ldap_configuration_templates:
        Option<Vec<::models::AuthLdapTemplatesLdapConfigurationTemplate>>,
}
