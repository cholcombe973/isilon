#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormProperties {
    /// Autocommit delay.
    #[serde(rename = "autocommit_delay")]
    pub autocommit_delay: Option<i32>,
    /// WORM domain ID.
    #[serde(rename = "domain_id")]
    pub domain_id: Option<i32>,
    /// WORM domain path.
    #[serde(rename = "domain_path")]
    pub domain_path: Option<String>,
    /// Indicates whether the file was committed to the WORM state.
    #[serde(rename = "worm_committed")]
    pub worm_committed: Option<bool>,
    /// WORM change time.
    #[serde(rename = "worm_ctime")]
    pub worm_ctime: Option<i32>,
    /// Provides the override retention date that is set on the SmartLock directory where the file resides.
    #[serde(rename = "worm_override_retention_date")]
    pub worm_override_retention_date: Option<String>,
    /// Provides the override retention date in seconds from UNIX Epoch or UTC.
    #[serde(rename = "worm_override_retention_date_val")]
    pub worm_override_retention_date_val: Option<i32>,
    /// Provides the retention expiration date in Coordinated Universal Time (such as UTC/GMT).
    #[serde(rename = "worm_retention_date")]
    pub worm_retention_date: Option<String>,
    /// Provides the retention expiration date in seconds from UNIX Epoch or UTC.
    #[serde(rename = "worm_retention_date_val")]
    pub worm_retention_date_val: Option<i32>,
}
