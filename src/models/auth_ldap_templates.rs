

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLdapTemplates {
  #[serde(rename = "ldap_field_template")]
  ldap_field_template: Option<Vec<::models::AuthLdapTemplatesLdapFieldTemplateItem>>
}

