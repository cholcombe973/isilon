/// ClusterIdentity : Unprivileged cluster information for display when logging in.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterIdentity {
    /// A description of the cluster.
    #[serde(rename = "description")]
    pub description: String,
    /// The information displayed when a user logs in to the cluster.
    #[serde(rename = "logon")]
    pub logon:crate::models::ClusterIdentityLogon,
    /// Enum to control the display message about the MTTDL of the cluster. This does NOT change the MTTDL of a cluster in anyway, and is purely a value for displaying messages.
    #[serde(rename = "mttdl_level_msg")]
    pub mttdl_level_msg: String,
    /// The name of the cluster.
    #[serde(rename = "name")]
    pub name: String,
}
