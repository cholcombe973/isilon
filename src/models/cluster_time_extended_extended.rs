#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterTimeExtendedExtended {
    /// The current time on the cluster as a UNIX epoch (seconds since 1/1/1970), as reported by this node.
    #[serde(rename = "time")]
    pub time: Option<i32>,
}
