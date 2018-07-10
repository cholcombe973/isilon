#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersLocalLocalItem {
    /// If true, enables authentication and identity management through the authentication provider.
    #[serde(rename = "authentication")]
    pub authentication: Option<bool>,
    /// Automatically creates the home directory on the first login.
    #[serde(rename = "create_home_directory")]
    pub create_home_directory: Option<bool>,
    /// Specifies the path to the home directory template.
    #[serde(rename = "home_directory_template")]
    pub home_directory_template: Option<String>,
    /// Specifies the local provider ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the length of time in seconds that an account will be inaccessible after multiple failed login attempts.
    #[serde(rename = "lockout_duration")]
    pub lockout_duration: Option<i32>,
    /// Specifies the number of failed login attempts necessary before an account is locked.
    #[serde(rename = "lockout_threshold")]
    pub lockout_threshold: Option<i32>,
    /// Specifies the duration of time in seconds in which the number of failed attempts set in the 'lockout_threshold' parameter must be made before an account is locked.
    #[serde(rename = "lockout_window")]
    pub lockout_window: Option<i32>,
    /// Specifies the login shell path.
    #[serde(rename = "login_shell")]
    pub login_shell: Option<String>,
    /// Specifies the domain for this provider through which users and groups are qualified.
    #[serde(rename = "machine_name")]
    pub machine_name: Option<String>,
    /// Specifies the maximum password age in seconds.
    #[serde(rename = "max_password_age")]
    pub max_password_age: Option<i32>,
    /// Specifies the minimum password age in seconds.
    #[serde(rename = "min_password_age")]
    pub min_password_age: Option<i32>,
    /// Specifies the minimum password length.
    #[serde(rename = "min_password_length")]
    pub min_password_length: Option<i32>,
    /// Specifies the local provider name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the conditions required for a password.
    #[serde(rename = "password_complexity")]
    pub password_complexity: Option<Vec<String>>,
    /// Specifies the number of previous passwords to store.
    #[serde(rename = "password_history_length")]
    pub password_history_length: Option<i32>,
    /// Specifies the time in seconds remaining before a user will be prompted for a password change.
    #[serde(rename = "password_prompt_time")]
    pub password_prompt_time: Option<i32>,
    /// Specifies the status of the provider.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// If true, indicates that this provider instance was created by OneFS and cannot be removed.
    #[serde(rename = "system")]
    pub system: Option<bool>,
}
