

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersKrb5Krb5ItemExtended {
  /// Groupnet identifier.
  #[serde(rename = "groupnet")]
  groupnet: Option<String>,
  /// Specifies the Kerberos provider ID.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies the key information for the Kerberos SPNs.
  #[serde(rename = "keytab_entries")]
  keytab_entries: Option<Vec<::models::ProvidersKrb5IdParamsKeytabEntry>>,
  /// Specifies the path to a keytab file to import.
  #[serde(rename = "keytab_file")]
  keytab_file: Option<String>,
  /// If true, keys are managed manually. If false, keys are managed through kadmin.
  #[serde(rename = "manual_keying")]
  manual_keying: Option<bool>,
  /// Specifies the Kerberos provider name.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specifies the name of realm.
  #[serde(rename = "realm")]
  realm: Option<String>,
  /// Specifies the recommended SPNs.
  #[serde(rename = "recommended_spns")]
  recommended_spns: Option<Vec<String>>,
  /// Specifies the status of the provider.
  #[serde(rename = "status")]
  status: Option<String>,
  /// If true, indicates that this provider instance was created by OneFS and cannot be removed
  #[serde(rename = "system")]
  system: Option<bool>,
  /// Specifies the name of the user that performs kadmin tasks.
  #[serde(rename = "user")]
  user: Option<String>,
  /// Specifies the Kerberos provider password.
  #[serde(rename = "password")]
  password: Option<String>
}

