#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudAccessCluster {
    /// A list of accounts created on the cluster with this guid
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<String>>,
    /// Whether the guid is that of the current (local) cluster
    #[serde(rename = "current")]
    pub current: Option<bool>,
    /// A cluster guid indicating the birth place of one or more accounts or policies on this cluster
    #[serde(rename = "guid")]
    pub guid: Option<String>,
    /// A cluster guid indicating the birth place of one or more accounts or policies on this cluster
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the cluster from which the above guid originated
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A list of policies created on the cluster with this guid
    #[serde(rename = "policies")]
    pub policies: Option<Vec<String>>,
    /// Whether the guid has access to the cloud
    #[serde(rename = "state")]
    pub state: Option<String>,
    /// The name of the cluster from which the above guid was synced
    #[serde(rename = "synced_from")]
    pub synced_from: Option<String>,
}
