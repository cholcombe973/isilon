
/// ClusterOwner : Cluster contact info settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterOwner {
  /// Cluster owner company name.
  #[serde(rename = "company")]
  company: Option<String>,
  /// Cluster owner location.
  #[serde(rename = "location")]
  location: Option<String>,
  /// Cluster owner primary email address.
  #[serde(rename = "primary_email")]
  primary_email: Option<String>,
  /// Cluster owner primary contact name.
  #[serde(rename = "primary_name")]
  primary_name: Option<String>,
  /// Cluster owner primary contact phone number 1.
  #[serde(rename = "primary_phone1")]
  primary_phone1: Option<String>,
  /// Cluster owner primary contact phone number 2.
  #[serde(rename = "primary_phone2")]
  primary_phone2: Option<String>,
  /// Cluster owner secondary email address.
  #[serde(rename = "secondary_email")]
  secondary_email: Option<String>,
  /// Cluster owner secondary contact name.
  #[serde(rename = "secondary_name")]
  secondary_name: Option<String>,
  /// Cluster owner secondary contact phone number 1.
  #[serde(rename = "secondary_phone1")]
  secondary_phone1: Option<String>,
  /// Cluster owner secondary contact phone number 2.
  #[serde(rename = "secondary_phone2")]
  secondary_phone2: Option<String>
}

