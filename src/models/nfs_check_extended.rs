/// NfsCheckExtended : Validation information about NFS exports.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsCheckExtended {
    #[serde(rename = "checks")]
    pub checks: Option<Vec <crate::models::NfsCheck>>,
}
