

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPolicySourceNetwork {
  /// The pool to restrict replication policies to.
  #[serde(rename = "pool")]
  pool: String,
  /// The subnet to restrict replication policies to.
  #[serde(rename = "subnet")]
  subnet: String
}

