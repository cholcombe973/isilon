#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersKrb5Krb5Item {
    /// Groupnet identifier.
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    /// Specifies the Kerberos provider ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the key information for the Kerberos SPNs.
    #[serde(rename = "keytab_entries")]
    pub keytab_entries: Option<Vec <crate::models::ProvidersKrb5IdParamsKeytabEntry>>,
    /// Specifies the path to a keytab file to import.
    #[serde(rename = "keytab_file")]
    pub keytab_file: Option<String>,
    /// If true, keys are managed manually. If false, keys are managed through kadmin.
    #[serde(rename = "manual_keying")]
    pub manual_keying: Option<bool>,
    /// Specifies the Kerberos provider name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the name of realm.
    #[serde(rename = "realm")]
    pub realm: Option<String>,
    /// Specifies the recommended SPNs.
    #[serde(rename = "recommended_spns")]
    pub recommended_spns: Option<Vec<String>>,
    /// Specifies the status of the provider.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// If true, indicates that this provider instance was created by OneFS and cannot be removed
    #[serde(rename = "system")]
    pub system: Option<bool>,
    /// Specifies the name of the user that performs kadmin tasks.
    #[serde(rename = "user")]
    pub user: Option<String>,
}
