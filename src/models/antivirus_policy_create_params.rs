/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusPolicyCreateParams {
  /// A description for the policy.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Whether the policy is enabled.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Forces the scan to run regardless of whether the files were recently scanned.
  #[serde(rename = "force_run")]
  force_run: Option<bool>,
  #[serde(rename = "impact")]
  impact: Option<String>,
  /// The name of the policy.
  #[serde(rename = "name")]
  name: String,
  /// Paths to include in the scan.
  #[serde(rename = "paths")]
  paths: Option<Vec<String>>,
  /// The depth to recurse in directories.  The default of -1 gives unlimited recursion.
  #[serde(rename = "recursion_depth")]
  recursion_depth: Option<i32>,
  #[serde(rename = "schedule")]
  schedule: Option<String>
}

impl AntivirusPolicyCreateParams {
  pub fn new(name: String) -> AntivirusPolicyCreateParams {
    AntivirusPolicyCreateParams {
      description: None,
      enabled: None,
      force_run: None,
      impact: None,
      name: name,
      paths: None,
      recursion_depth: None,
      schedule: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> AntivirusPolicyCreateParams {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> AntivirusPolicyCreateParams {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_force_run(&mut self, force_run: bool) {
    self.force_run = Some(force_run);
  }

  pub fn with_force_run(mut self, force_run: bool) -> AntivirusPolicyCreateParams {
    self.force_run = Some(force_run);
    self
  }

  pub fn force_run(&self) -> Option<&bool> {
    self.force_run.as_ref()
  }

  pub fn reset_force_run(&mut self) {
    self.force_run = None;
  }

  pub fn set_impact(&mut self, impact: String) {
    self.impact = Some(impact);
  }

  pub fn with_impact(mut self, impact: String) -> AntivirusPolicyCreateParams {
    self.impact = Some(impact);
    self
  }

  pub fn impact(&self) -> Option<&String> {
    self.impact.as_ref()
  }

  pub fn reset_impact(&mut self) {
    self.impact = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> AntivirusPolicyCreateParams {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_paths(&mut self, paths: Vec<String>) {
    self.paths = Some(paths);
  }

  pub fn with_paths(mut self, paths: Vec<String>) -> AntivirusPolicyCreateParams {
    self.paths = Some(paths);
    self
  }

  pub fn paths(&self) -> Option<&Vec<String>> {
    self.paths.as_ref()
  }

  pub fn reset_paths(&mut self) {
    self.paths = None;
  }

  pub fn set_recursion_depth(&mut self, recursion_depth: i32) {
    self.recursion_depth = Some(recursion_depth);
  }

  pub fn with_recursion_depth(mut self, recursion_depth: i32) -> AntivirusPolicyCreateParams {
    self.recursion_depth = Some(recursion_depth);
    self
  }

  pub fn recursion_depth(&self) -> Option<&i32> {
    self.recursion_depth.as_ref()
  }

  pub fn reset_recursion_depth(&mut self) {
    self.recursion_depth = None;
  }

  pub fn set_schedule(&mut self, schedule: String) {
    self.schedule = Some(schedule);
  }

  pub fn with_schedule(mut self, schedule: String) -> AntivirusPolicyCreateParams {
    self.schedule = Some(schedule);
    self
  }

  pub fn schedule(&self) -> Option<&String> {
    self.schedule.as_ref()
  }

  pub fn reset_schedule(&mut self) {
    self.schedule = None;
  }

}


