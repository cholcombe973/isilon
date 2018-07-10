
/// AuthUser : Specifies the configuration properties for a user.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
  /// Specifies an email address for the user.
  #[serde(rename = "email")]
  email: Option<String>,
  /// If true, the authenticated user is enabled.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Specifies the Unix Epoch time when the auth user will expire.
  #[serde(rename = "expiry")]
  expiry: Option<i32>,
  /// Specifies the GECOS value, which is usually the full name.
  #[serde(rename = "gecos")]
  gecos: Option<String>,
  /// Specifies a home directory for the user.
  #[serde(rename = "home_directory")]
  home_directory: Option<String>,
  /// Changes the password for the user.
  #[serde(rename = "password")]
  password: Option<String>,
  /// If true, the password should expire.
  #[serde(rename = "password_expires")]
  password_expires: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "primary_group")]
  primary_group: Option<::models::AuthAccessAccessItemFileGroup>,
  /// If true, prompts the user to change their password at the next login.
  #[serde(rename = "prompt_password_change")]
  prompt_password_change: Option<bool>,
  /// Specifies the shell for the user.
  #[serde(rename = "shell")]
  shell: Option<String>,
  /// Specifies a security identifier.
  #[serde(rename = "sid")]
  sid: Option<String>,
  /// Specifies a numeric user identifier.
  #[serde(rename = "uid")]
  uid: Option<i32>,
  /// If true, the user account should be unlocked.
  #[serde(rename = "unlock")]
  unlock: Option<bool>
}

