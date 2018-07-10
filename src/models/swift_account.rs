/// SwiftAccount : This is an account for the Swift protocol.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SwiftAccount {
    /// Unique id of swift account
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of Swift account
    #[serde(rename = "name")]
    pub name: String,
    /// Group with filesystem ownership of this account
    #[serde(rename = "swiftgroup")]
    pub swiftgroup: Option<String>,
    /// User with filesystem ownership of this account
    #[serde(rename = "swiftuser")]
    pub swiftuser: Option<String>,
    /// Users who are allowed to access Swift account
    #[serde(rename = "users")]
    pub users: Option<Vec<String>>,
    /// Name of access zone for account
    #[serde(rename = "zone")]
    pub zone: Option<String>,
}
