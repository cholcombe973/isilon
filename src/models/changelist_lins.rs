#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelistLins {
    ///
    #[serde(rename = "atime")]
    pub atime: Option <crate::models::ChangelistLinsCtime>,
    ///
    #[serde(rename = "ctime")]
    pub ctime: Option <crate::models::ChangelistLinsCtime>,
    /// The LIN number of the file associated with the entry.
    #[serde(rename = "id")]
    pub id: String,
    ///
    #[serde(rename = "mtime")]
    pub mtime: Option <crate::models::ChangelistLinsCtime>,
    /// The path to the file associated with the entry.
    #[serde(rename = "path")]
    pub path: String,
    /// The size of the file associated with the entry.
    #[serde(rename = "size")]
    pub size: i32,
    /// Type of file.
    #[serde(rename = "type")]
    pub _type: String,
}
