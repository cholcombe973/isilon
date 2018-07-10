
/// NfsCheckExtended : Validation information about NFS exports.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsCheckExtended {
  #[serde(rename = "checks")]
  checks: Option<Vec<::models::NfsCheck>>
}

