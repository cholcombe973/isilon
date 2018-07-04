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
pub struct WormProperties {
  /// Autocommit delay.
  #[serde(rename = "autocommit_delay")]
  autocommit_delay: Option<i32>,
  /// WORM domain ID.
  #[serde(rename = "domain_id")]
  domain_id: Option<i32>,
  /// WORM domain path.
  #[serde(rename = "domain_path")]
  domain_path: Option<String>,
  /// Indicates whether the file was committed to the WORM state.
  #[serde(rename = "worm_committed")]
  worm_committed: Option<bool>,
  /// WORM change time.
  #[serde(rename = "worm_ctime")]
  worm_ctime: Option<i32>,
  /// Provides the override retention date that is set on the SmartLock directory where the file resides.
  #[serde(rename = "worm_override_retention_date")]
  worm_override_retention_date: Option<String>,
  /// Provides the override retention date in seconds from UNIX Epoch or UTC.
  #[serde(rename = "worm_override_retention_date_val")]
  worm_override_retention_date_val: Option<i32>,
  /// Provides the retention expiration date in Coordinated Universal Time (such as UTC/GMT).
  #[serde(rename = "worm_retention_date")]
  worm_retention_date: Option<String>,
  /// Provides the retention expiration date in seconds from UNIX Epoch or UTC.
  #[serde(rename = "worm_retention_date_val")]
  worm_retention_date_val: Option<i32>
}

impl WormProperties {
  pub fn new() -> WormProperties {
    WormProperties {
      autocommit_delay: None,
      domain_id: None,
      domain_path: None,
      worm_committed: None,
      worm_ctime: None,
      worm_override_retention_date: None,
      worm_override_retention_date_val: None,
      worm_retention_date: None,
      worm_retention_date_val: None
    }
  }

  pub fn set_autocommit_delay(&mut self, autocommit_delay: i32) {
    self.autocommit_delay = Some(autocommit_delay);
  }

  pub fn with_autocommit_delay(mut self, autocommit_delay: i32) -> WormProperties {
    self.autocommit_delay = Some(autocommit_delay);
    self
  }

  pub fn autocommit_delay(&self) -> Option<&i32> {
    self.autocommit_delay.as_ref()
  }

  pub fn reset_autocommit_delay(&mut self) {
    self.autocommit_delay = None;
  }

  pub fn set_domain_id(&mut self, domain_id: i32) {
    self.domain_id = Some(domain_id);
  }

  pub fn with_domain_id(mut self, domain_id: i32) -> WormProperties {
    self.domain_id = Some(domain_id);
    self
  }

  pub fn domain_id(&self) -> Option<&i32> {
    self.domain_id.as_ref()
  }

  pub fn reset_domain_id(&mut self) {
    self.domain_id = None;
  }

  pub fn set_domain_path(&mut self, domain_path: String) {
    self.domain_path = Some(domain_path);
  }

  pub fn with_domain_path(mut self, domain_path: String) -> WormProperties {
    self.domain_path = Some(domain_path);
    self
  }

  pub fn domain_path(&self) -> Option<&String> {
    self.domain_path.as_ref()
  }

  pub fn reset_domain_path(&mut self) {
    self.domain_path = None;
  }

  pub fn set_worm_committed(&mut self, worm_committed: bool) {
    self.worm_committed = Some(worm_committed);
  }

  pub fn with_worm_committed(mut self, worm_committed: bool) -> WormProperties {
    self.worm_committed = Some(worm_committed);
    self
  }

  pub fn worm_committed(&self) -> Option<&bool> {
    self.worm_committed.as_ref()
  }

  pub fn reset_worm_committed(&mut self) {
    self.worm_committed = None;
  }

  pub fn set_worm_ctime(&mut self, worm_ctime: i32) {
    self.worm_ctime = Some(worm_ctime);
  }

  pub fn with_worm_ctime(mut self, worm_ctime: i32) -> WormProperties {
    self.worm_ctime = Some(worm_ctime);
    self
  }

  pub fn worm_ctime(&self) -> Option<&i32> {
    self.worm_ctime.as_ref()
  }

  pub fn reset_worm_ctime(&mut self) {
    self.worm_ctime = None;
  }

  pub fn set_worm_override_retention_date(&mut self, worm_override_retention_date: String) {
    self.worm_override_retention_date = Some(worm_override_retention_date);
  }

  pub fn with_worm_override_retention_date(mut self, worm_override_retention_date: String) -> WormProperties {
    self.worm_override_retention_date = Some(worm_override_retention_date);
    self
  }

  pub fn worm_override_retention_date(&self) -> Option<&String> {
    self.worm_override_retention_date.as_ref()
  }

  pub fn reset_worm_override_retention_date(&mut self) {
    self.worm_override_retention_date = None;
  }

  pub fn set_worm_override_retention_date_val(&mut self, worm_override_retention_date_val: i32) {
    self.worm_override_retention_date_val = Some(worm_override_retention_date_val);
  }

  pub fn with_worm_override_retention_date_val(mut self, worm_override_retention_date_val: i32) -> WormProperties {
    self.worm_override_retention_date_val = Some(worm_override_retention_date_val);
    self
  }

  pub fn worm_override_retention_date_val(&self) -> Option<&i32> {
    self.worm_override_retention_date_val.as_ref()
  }

  pub fn reset_worm_override_retention_date_val(&mut self) {
    self.worm_override_retention_date_val = None;
  }

  pub fn set_worm_retention_date(&mut self, worm_retention_date: String) {
    self.worm_retention_date = Some(worm_retention_date);
  }

  pub fn with_worm_retention_date(mut self, worm_retention_date: String) -> WormProperties {
    self.worm_retention_date = Some(worm_retention_date);
    self
  }

  pub fn worm_retention_date(&self) -> Option<&String> {
    self.worm_retention_date.as_ref()
  }

  pub fn reset_worm_retention_date(&mut self) {
    self.worm_retention_date = None;
  }

  pub fn set_worm_retention_date_val(&mut self, worm_retention_date_val: i32) {
    self.worm_retention_date_val = Some(worm_retention_date_val);
  }

  pub fn with_worm_retention_date_val(mut self, worm_retention_date_val: i32) -> WormProperties {
    self.worm_retention_date_val = Some(worm_retention_date_val);
    self
  }

  pub fn worm_retention_date_val(&self) -> Option<&i32> {
    self.worm_retention_date_val.as_ref()
  }

  pub fn reset_worm_retention_date_val(&mut self) {
    self.worm_retention_date_val = None;
  }

}


