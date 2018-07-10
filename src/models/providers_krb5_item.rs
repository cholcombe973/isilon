/// ProvidersKrb5Item : Specifies properties for the Kerberos authentication provider.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersKrb5Item {
    /// Groupnet identifier.
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    /// Specifies the key information for the Kerberos SPN.
    #[serde(rename = "keytab_entries")]
    pub keytab_entries: Option<Vec<::models::ProvidersKrb5IdParamsKeytabEntry>>,
    /// Specifies the path to a keytab file to import.
    #[serde(rename = "keytab_file")]
    pub keytab_file: Option<String>,
    /// If true, keys are managed manually. If false, keys are managed through kadmin.
    #[serde(rename = "manual_keying")]
    pub manual_keying: Option<bool>,
    /// Specifies the Kerberos provider name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the Kerberos provider password.
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// Specifies the name of realm.
    #[serde(rename = "realm")]
    pub realm: String,
    /// Specifies the name of the user that performs kadmin tasks.
    #[serde(rename = "user")]
    pub user: Option<String>,
}
