#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceObject {
    /// Specifies the date when the object was last accessed in HTTP date/time format.
    #[serde(rename = "access_time")]
    pub access_time: Option<String>,
    /// Specifies the time when the object was last accessed in UNIX Epoch format.
    #[serde(rename = "atime_val")]
    pub atime_val: Option<i32>,
    /// Specifies the block size of the object.
    #[serde(rename = "block_size")]
    pub block_size: Option<i32>,
    /// Specifies the number of blocks that compose the object.
    #[serde(rename = "blocks")]
    pub blocks: Option<i32>,
    /// Specifies the time when the object data was created in UNIX Epoch format.
    #[serde(rename = "btime_val")]
    pub btime_val: Option<i32>,
    /// Specifies the date when the object was last changed (including data and metadata changes) in HTTP date/time format.
    #[serde(rename = "change_time")]
    pub change_time: Option<String>,
    /// Specifies the date when the object data was created in HTTP date/time format.
    #[serde(rename = "create_time")]
    pub create_time: Option<String>,
    /// Specifies the time when the object was last changed (including data and metadata changes) in UNIX Epoch format.
    #[serde(rename = "ctime_val")]
    pub ctime_val: Option<i32>,
    /// Specifies the GID for the owner.
    #[serde(rename = "gid")]
    pub gid: Option<i32>,
    /// Specifies the group name for the owner of the object.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// Specifies the object ID, which is also the INODE number.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Specifies whether the file is hidden or not.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// Specifies the time when the object data was last modified in HTTP date/time format.
    #[serde(rename = "last_modified")]
    pub last_modified: Option<String>,
    /// Specifies the UNIX mode octal number.
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    /// Specifies the time when the object data was last modified in UNIX Epoch format.
    #[serde(rename = "mtime_val")]
    pub mtime_val: Option<i32>,
    /// Specifies the name of the object.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the number of hard links to the object.
    #[serde(rename = "nlink")]
    pub nlink: Option<i32>,
    /// Specifies the user name for the owner of the object.
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    /// Specifies the size of the object in bytes.
    #[serde(rename = "size")]
    pub size: Option<i32>,
    #[serde(rename = "stub")]
    pub stub: Option<bool>,
    /// Specifies the object type, which can be one of the following values: container, object, pipe, character_device, block_device, symbolic_link, socket, or whiteout_file.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Specifies the UID for the owner.
    #[serde(rename = "uid")]
    pub uid: Option<i32>,
}
