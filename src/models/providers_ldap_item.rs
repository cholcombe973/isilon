/// ProvidersLdapItem : Specifies the properties for the LDAP authentication provider.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersLdapItem {
    /// Specifies the attribute name used when searching for alternate security identities.
    #[serde(rename = "alternate_security_identities_attribute")]
    pub alternate_security_identities_attribute: Option<String>,
    /// If true, enables authentication and identity management through the authentication provider.
    #[serde(rename = "authentication")]
    pub authentication: Option<bool>,
    /// If true, connects the provider to a random server.
    #[serde(rename = "balance_servers")]
    pub balance_servers: Option<bool>,
    /// Specifies the root of the tree in which to search identities.
    #[serde(rename = "base_dn")]
    pub base_dn: String,
    /// Specifies the distinguished name for binding to the LDAP server.
    #[serde(rename = "bind_dn")]
    pub bind_dn: Option<String>,
}
