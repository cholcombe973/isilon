

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersLdap {
  #[serde(rename = "ldap")]
  ldap: Option<Vec<::models::ProvidersLdapLdapItem>>
}

