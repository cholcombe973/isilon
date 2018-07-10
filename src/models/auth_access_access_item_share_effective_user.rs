#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItemShareEffectiveUser {
    /// Specifies the serialized form of the persona, which can be 'UID:0', 'USER:name', 'GID:0', 'GROUP:wheel', or 'SID:S-1-1'.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the persona name, which must be combined with a type.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the type, which must be combined with a name.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Specifies the uid of the user.
    #[serde(rename = "uid")]
    pub uid: Option<i32>,
}
