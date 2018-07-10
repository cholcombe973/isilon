/// NfsAlias : Specifies properties for NFS aliases, which are names for physical paths in the file system.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsAlias {
    /// Specifies whether the alias is usable.
    #[serde(rename = "health")]
    pub health: Option<String>,
    /// Specifies the name by which the alias can be referenced.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the path to which the alias points.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Specifies the zone in which the alias is valid.
    #[serde(rename = "zone")]
    pub zone: Option<String>,
}
