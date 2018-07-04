/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ProvidersLocalIdParams : Specifies the properties for a local authentication provider.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersLocalIdParams {
  /// If true, enables authentication and identity management through the authentication provider.
  #[serde(rename = "authentication")]
  authentication: Option<bool>,
  /// Automatically creates the home directory on the first login.
  #[serde(rename = "create_home_directory")]
  create_home_directory: Option<bool>,
  /// Specifies the path to the home directory template.
  #[serde(rename = "home_directory_template")]
  home_directory_template: Option<String>,
  /// Specifies the length of time in seconds that an account will be inaccessible after multiple failed login attempts.
  #[serde(rename = "lockout_duration")]
  lockout_duration: Option<i32>,
  /// Specifies the number of failed login attempts necessary before an account is locked.
  #[serde(rename = "lockout_threshold")]
  lockout_threshold: Option<i32>,
  /// Specifies the duration of time in seconds in which the number of failed attempts set in the 'lockout_threshold' parameter must be made before an account is locked.
  #[serde(rename = "lockout_window")]
  lockout_window: Option<i32>,
  /// Specifies the login shell path.
  #[serde(rename = "login_shell")]
  login_shell: Option<String>,
  /// Specifies the domain for this provider through which users and groups are qualified.
  #[serde(rename = "machine_name")]
  machine_name: Option<String>,
  /// Specifies the maximum password age in seconds.
  #[serde(rename = "max_password_age")]
  max_password_age: Option<i32>,
  /// Specifies the minimum password age in seconds.
  #[serde(rename = "min_password_age")]
  min_password_age: Option<i32>,
  /// Specifies the minimum password length.
  #[serde(rename = "min_password_length")]
  min_password_length: Option<i32>,
  /// Specifies the local provider name.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specifies the conditions required for a password.
  #[serde(rename = "password_complexity")]
  password_complexity: Option<Vec<String>>,
  /// Specifies the number of previous passwords to store.
  #[serde(rename = "password_history_length")]
  password_history_length: Option<i32>,
  /// Specifies the time in seconds remaining before a user will be prompted for a password change.
  #[serde(rename = "password_prompt_time")]
  password_prompt_time: Option<i32>
}

impl ProvidersLocalIdParams {
  /// Specifies the properties for a local authentication provider.
  pub fn new() -> ProvidersLocalIdParams {
    ProvidersLocalIdParams {
      authentication: None,
      create_home_directory: None,
      home_directory_template: None,
      lockout_duration: None,
      lockout_threshold: None,
      lockout_window: None,
      login_shell: None,
      machine_name: None,
      max_password_age: None,
      min_password_age: None,
      min_password_length: None,
      name: None,
      password_complexity: None,
      password_history_length: None,
      password_prompt_time: None
    }
  }

  pub fn set_authentication(&mut self, authentication: bool) {
    self.authentication = Some(authentication);
  }

  pub fn with_authentication(mut self, authentication: bool) -> ProvidersLocalIdParams {
    self.authentication = Some(authentication);
    self
  }

  pub fn authentication(&self) -> Option<&bool> {
    self.authentication.as_ref()
  }

  pub fn reset_authentication(&mut self) {
    self.authentication = None;
  }

  pub fn set_create_home_directory(&mut self, create_home_directory: bool) {
    self.create_home_directory = Some(create_home_directory);
  }

  pub fn with_create_home_directory(mut self, create_home_directory: bool) -> ProvidersLocalIdParams {
    self.create_home_directory = Some(create_home_directory);
    self
  }

  pub fn create_home_directory(&self) -> Option<&bool> {
    self.create_home_directory.as_ref()
  }

  pub fn reset_create_home_directory(&mut self) {
    self.create_home_directory = None;
  }

  pub fn set_home_directory_template(&mut self, home_directory_template: String) {
    self.home_directory_template = Some(home_directory_template);
  }

  pub fn with_home_directory_template(mut self, home_directory_template: String) -> ProvidersLocalIdParams {
    self.home_directory_template = Some(home_directory_template);
    self
  }

  pub fn home_directory_template(&self) -> Option<&String> {
    self.home_directory_template.as_ref()
  }

  pub fn reset_home_directory_template(&mut self) {
    self.home_directory_template = None;
  }

  pub fn set_lockout_duration(&mut self, lockout_duration: i32) {
    self.lockout_duration = Some(lockout_duration);
  }

  pub fn with_lockout_duration(mut self, lockout_duration: i32) -> ProvidersLocalIdParams {
    self.lockout_duration = Some(lockout_duration);
    self
  }

  pub fn lockout_duration(&self) -> Option<&i32> {
    self.lockout_duration.as_ref()
  }

  pub fn reset_lockout_duration(&mut self) {
    self.lockout_duration = None;
  }

  pub fn set_lockout_threshold(&mut self, lockout_threshold: i32) {
    self.lockout_threshold = Some(lockout_threshold);
  }

  pub fn with_lockout_threshold(mut self, lockout_threshold: i32) -> ProvidersLocalIdParams {
    self.lockout_threshold = Some(lockout_threshold);
    self
  }

  pub fn lockout_threshold(&self) -> Option<&i32> {
    self.lockout_threshold.as_ref()
  }

  pub fn reset_lockout_threshold(&mut self) {
    self.lockout_threshold = None;
  }

  pub fn set_lockout_window(&mut self, lockout_window: i32) {
    self.lockout_window = Some(lockout_window);
  }

  pub fn with_lockout_window(mut self, lockout_window: i32) -> ProvidersLocalIdParams {
    self.lockout_window = Some(lockout_window);
    self
  }

  pub fn lockout_window(&self) -> Option<&i32> {
    self.lockout_window.as_ref()
  }

  pub fn reset_lockout_window(&mut self) {
    self.lockout_window = None;
  }

  pub fn set_login_shell(&mut self, login_shell: String) {
    self.login_shell = Some(login_shell);
  }

  pub fn with_login_shell(mut self, login_shell: String) -> ProvidersLocalIdParams {
    self.login_shell = Some(login_shell);
    self
  }

  pub fn login_shell(&self) -> Option<&String> {
    self.login_shell.as_ref()
  }

  pub fn reset_login_shell(&mut self) {
    self.login_shell = None;
  }

  pub fn set_machine_name(&mut self, machine_name: String) {
    self.machine_name = Some(machine_name);
  }

  pub fn with_machine_name(mut self, machine_name: String) -> ProvidersLocalIdParams {
    self.machine_name = Some(machine_name);
    self
  }

  pub fn machine_name(&self) -> Option<&String> {
    self.machine_name.as_ref()
  }

  pub fn reset_machine_name(&mut self) {
    self.machine_name = None;
  }

  pub fn set_max_password_age(&mut self, max_password_age: i32) {
    self.max_password_age = Some(max_password_age);
  }

  pub fn with_max_password_age(mut self, max_password_age: i32) -> ProvidersLocalIdParams {
    self.max_password_age = Some(max_password_age);
    self
  }

  pub fn max_password_age(&self) -> Option<&i32> {
    self.max_password_age.as_ref()
  }

  pub fn reset_max_password_age(&mut self) {
    self.max_password_age = None;
  }

  pub fn set_min_password_age(&mut self, min_password_age: i32) {
    self.min_password_age = Some(min_password_age);
  }

  pub fn with_min_password_age(mut self, min_password_age: i32) -> ProvidersLocalIdParams {
    self.min_password_age = Some(min_password_age);
    self
  }

  pub fn min_password_age(&self) -> Option<&i32> {
    self.min_password_age.as_ref()
  }

  pub fn reset_min_password_age(&mut self) {
    self.min_password_age = None;
  }

  pub fn set_min_password_length(&mut self, min_password_length: i32) {
    self.min_password_length = Some(min_password_length);
  }

  pub fn with_min_password_length(mut self, min_password_length: i32) -> ProvidersLocalIdParams {
    self.min_password_length = Some(min_password_length);
    self
  }

  pub fn min_password_length(&self) -> Option<&i32> {
    self.min_password_length.as_ref()
  }

  pub fn reset_min_password_length(&mut self) {
    self.min_password_length = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ProvidersLocalIdParams {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_password_complexity(&mut self, password_complexity: Vec<String>) {
    self.password_complexity = Some(password_complexity);
  }

  pub fn with_password_complexity(mut self, password_complexity: Vec<String>) -> ProvidersLocalIdParams {
    self.password_complexity = Some(password_complexity);
    self
  }

  pub fn password_complexity(&self) -> Option<&Vec<String>> {
    self.password_complexity.as_ref()
  }

  pub fn reset_password_complexity(&mut self) {
    self.password_complexity = None;
  }

  pub fn set_password_history_length(&mut self, password_history_length: i32) {
    self.password_history_length = Some(password_history_length);
  }

  pub fn with_password_history_length(mut self, password_history_length: i32) -> ProvidersLocalIdParams {
    self.password_history_length = Some(password_history_length);
    self
  }

  pub fn password_history_length(&self) -> Option<&i32> {
    self.password_history_length.as_ref()
  }

  pub fn reset_password_history_length(&mut self) {
    self.password_history_length = None;
  }

  pub fn set_password_prompt_time(&mut self, password_prompt_time: i32) {
    self.password_prompt_time = Some(password_prompt_time);
  }

  pub fn with_password_prompt_time(mut self, password_prompt_time: i32) -> ProvidersLocalIdParams {
    self.password_prompt_time = Some(password_prompt_time);
    self
  }

  pub fn password_prompt_time(&self) -> Option<&i32> {
    self.password_prompt_time.as_ref()
  }

  pub fn reset_password_prompt_time(&mut self) {
    self.password_prompt_time = None;
  }

}


