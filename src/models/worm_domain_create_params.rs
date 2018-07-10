#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormDomainCreateParams {
    /// Specifies the autocommit time period for the domain in seconds.  After a file is in the domain without being modified for the specified time period, the file is automatically committed. If this parameter is set to null, there is no autocommit time, and files must be committed manually.
    #[serde(rename = "autocommit_offset")]
    pub autocommit_offset: Option<i32>,
    #[serde(rename = "default_retention")]
    pub default_retention: Option<String>,
    #[serde(rename = "max_retention")]
    pub max_retention: Option<String>,
    #[serde(rename = "min_retention")]
    pub min_retention: Option<String>,
    /// Specifies the override retention date for the domain. If this date is later than the retention date for any committed file, the file will remain protected until the override retention date.
    #[serde(rename = "override_date")]
    pub override_date: Option<i32>,
    /// When this value is set to 'on', files in this domain can be deleted through the privileged delete feature. If this value is set to 'disabled', privileged file deletes are permanently disabled and cannot be turned on again.
    #[serde(rename = "privileged_delete")]
    pub privileged_delete: Option<String>,
    /// Specifies whether the domain is an enterprise domain or a compliance domain. Compliance domains can not be created on enterprise clusters. Enterprise and compliance domains can be created on compliance clusters.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Specifies the root path of this domain. Files in this directory and all sub-directories will be protected.
    #[serde(rename = "path")]
    pub path: String,
}
