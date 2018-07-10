

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItemFile {
  /// Specifies absolute path in filesystem.
  #[serde(rename = "effective_path")]
  effective_path: Option<String>,
  /// Specifies the permissions that the user has on the file.
  #[serde(rename = "file_permissions")]
  file_permissions: Option<::models::AuthAccessAccessItemFileFilePermissions>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "group")]
  group: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies whether path is inside snapshot.
  #[serde(rename = "is_snapshot")]
  is_snapshot: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "owner")]
  owner: Option<::models::AuthAccessAccessItemFileGroup>
}

