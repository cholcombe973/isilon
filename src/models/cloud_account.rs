
/// CloudAccount : A cloud account object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudAccount {
  /// (S3 only) The user id of the S3 account
  #[serde(rename = "account_id")]
  account_id: Option<String>,
  /// The username required to authenticate against the cloud service
  #[serde(rename = "account_username")]
  account_username: Option<String>,
  /// The guid of the cluster where this account was created
  #[serde(rename = "birth_cluster_id")]
  birth_cluster_id: Option<String>,
  /// Whether this account is explicitly enabled or disabled by a user
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// A valid authentication key for connecting to the cloud
  #[serde(rename = "key")]
  key: Option<String>,
  /// A unique name for this account
  #[serde(rename = "name")]
  name: Option<String>,
  /// The id or name of a proxy to be used by this account
  #[serde(rename = "proxy")]
  proxy: Option<String>,
  /// (Not recommended) Indicates whether to skip validation that the cloud account is still accessible
  #[serde(rename = "skip_account_check")]
  skip_account_check: Option<bool>,
  /// Indicates whether to skip SSL certificate validation when connecting to the cloud
  #[serde(rename = "skip_ssl_validation")]
  skip_ssl_validation: Option<bool>,
  /// (S3 only) An appropriate region for the S3 account.  For example, faster access times may be gained by referencing a nearby region
  #[serde(rename = "storage_region")]
  storage_region: Option<String>,
  /// (S3 only) The name of the bucket into which generated metrics reports are placed by the cloud service provider
  #[serde(rename = "telemetry_bucket")]
  telemetry_bucket: Option<String>,
  /// A valid URI pointing to the location of the cloud storage
  #[serde(rename = "uri")]
  uri: Option<String>
}

