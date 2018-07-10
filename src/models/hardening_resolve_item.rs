
/// HardeningResolveItem : Resolve Issues found on the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningResolveItem {
  /// Hardening profile.
  #[serde(rename = "profile")]
  profile: Option<String>
}

