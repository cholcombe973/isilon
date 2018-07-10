/// CloudPool : A group of cloud accounts which will be the destination for file stubbing

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudPool {
    /// A list of valid names for the accounts in this pool.  There is currently only one account allowed per pool.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<String>>,
    /// The guid of the cluster where this pool was created
    #[serde(rename = "birth_cluster_id")]
    pub birth_cluster_id: Option<String>,
    /// A brief description of this pool
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A unique name for this pool
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A string identifier of the cloud services vendor
    #[serde(rename = "vendor")]
    pub vendor: Option<String>,
}
