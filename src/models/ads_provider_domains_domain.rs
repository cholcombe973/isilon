#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdsProviderDomainsDomain {
    #[serde(rename = "client_site")]
    pub client_site: Option<String>,
    #[serde(rename = "dc_address")]
    pub dc_address: Option<String>,
    #[serde(rename = "dc_name")]
    pub dc_name: Option<String>,
    #[serde(rename = "dc_site")]
    pub dc_site: Option<String>,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "guid")]
    pub guid: Option<String>,
    /// Specifies a unique identifier for every domain returned.
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "netbios_name")]
    pub netbios_name: Option<String>,
    #[serde(rename = "sid")]
    pub sid: Option<String>,
    /// Specifies the status of the domain.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Specifies the type of trust for this domain. Options include 'primary', 'unknown', 'external', and 'forest'.
    #[serde(rename = "trust_type")]
    pub trust_type: Option<String>,
}
