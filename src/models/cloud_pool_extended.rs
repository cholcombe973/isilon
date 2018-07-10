

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudPoolExtended {
  /// A list of valid names for the accounts in this pool.  There is currently only one account allowed per pool.
  #[serde(rename = "accounts")]
  accounts: Option<Vec<String>>,
  /// The guid of the cluster where this pool was created
  #[serde(rename = "birth_cluster_id")]
  birth_cluster_id: Option<String>,
  /// A brief description of this pool
  #[serde(rename = "description")]
  description: Option<String>,
  /// A unique name for this pool
  #[serde(rename = "name")]
  name: Option<String>,
  /// A string identifier of the cloud services vendor
  #[serde(rename = "vendor")]
  vendor: Option<String>,
  /// A unique name for this pool
  #[serde(rename = "id")]
  id: Option<String>,
  /// Indicates whether this pool is in a good state (\"OK\") or disabled (\"disabled\")
  #[serde(rename = "state")]
  state: Option<String>,
  /// Gives further information to describe the state of this pool
  #[serde(rename = "state_details")]
  state_details: Option<String>,
  /// The type of cloud protocol required.  E.g., \"isilon\" for EMC Isilon, \"ecs\" for EMC ECS Appliance, \"virtustream\" for Virtustream Storage Cloud, \"azure\" for Microsoft Azure and \"s3\" for Amazon S3
  #[serde(rename = "type")]
  _type: Option<String>
}

