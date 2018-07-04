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
pub struct NfsSettingsGlobalSettings {
  /// True if NFSv3 is enabled.
  #[serde(rename = "nfsv3_enabled")]
  nfsv3_enabled: Option<bool>,
  /// True if NFSv4 is enabled.
  #[serde(rename = "nfsv4_enabled")]
  nfsv4_enabled: Option<bool>,
  /// Specifies the maximum number of threads in the nfsd thread pool.
  #[serde(rename = "rpc_maxthreads")]
  rpc_maxthreads: Option<i32>,
  /// Specifies the minimum number of threads in the nfsd thread pool.
  #[serde(rename = "rpc_minthreads")]
  rpc_minthreads: Option<i32>,
  /// True if the NFS service is enabled. When set to false, the NFS service is disabled.
  #[serde(rename = "service")]
  service: Option<bool>
}

impl NfsSettingsGlobalSettings {
  pub fn new() -> NfsSettingsGlobalSettings {
    NfsSettingsGlobalSettings {
      nfsv3_enabled: None,
      nfsv4_enabled: None,
      rpc_maxthreads: None,
      rpc_minthreads: None,
      service: None
    }
  }

  pub fn set_nfsv3_enabled(&mut self, nfsv3_enabled: bool) {
    self.nfsv3_enabled = Some(nfsv3_enabled);
  }

  pub fn with_nfsv3_enabled(mut self, nfsv3_enabled: bool) -> NfsSettingsGlobalSettings {
    self.nfsv3_enabled = Some(nfsv3_enabled);
    self
  }

  pub fn nfsv3_enabled(&self) -> Option<&bool> {
    self.nfsv3_enabled.as_ref()
  }

  pub fn reset_nfsv3_enabled(&mut self) {
    self.nfsv3_enabled = None;
  }

  pub fn set_nfsv4_enabled(&mut self, nfsv4_enabled: bool) {
    self.nfsv4_enabled = Some(nfsv4_enabled);
  }

  pub fn with_nfsv4_enabled(mut self, nfsv4_enabled: bool) -> NfsSettingsGlobalSettings {
    self.nfsv4_enabled = Some(nfsv4_enabled);
    self
  }

  pub fn nfsv4_enabled(&self) -> Option<&bool> {
    self.nfsv4_enabled.as_ref()
  }

  pub fn reset_nfsv4_enabled(&mut self) {
    self.nfsv4_enabled = None;
  }

  pub fn set_rpc_maxthreads(&mut self, rpc_maxthreads: i32) {
    self.rpc_maxthreads = Some(rpc_maxthreads);
  }

  pub fn with_rpc_maxthreads(mut self, rpc_maxthreads: i32) -> NfsSettingsGlobalSettings {
    self.rpc_maxthreads = Some(rpc_maxthreads);
    self
  }

  pub fn rpc_maxthreads(&self) -> Option<&i32> {
    self.rpc_maxthreads.as_ref()
  }

  pub fn reset_rpc_maxthreads(&mut self) {
    self.rpc_maxthreads = None;
  }

  pub fn set_rpc_minthreads(&mut self, rpc_minthreads: i32) {
    self.rpc_minthreads = Some(rpc_minthreads);
  }

  pub fn with_rpc_minthreads(mut self, rpc_minthreads: i32) -> NfsSettingsGlobalSettings {
    self.rpc_minthreads = Some(rpc_minthreads);
    self
  }

  pub fn rpc_minthreads(&self) -> Option<&i32> {
    self.rpc_minthreads.as_ref()
  }

  pub fn reset_rpc_minthreads(&mut self) {
    self.rpc_minthreads = None;
  }

  pub fn set_service(&mut self, service: bool) {
    self.service = Some(service);
  }

  pub fn with_service(mut self, service: bool) -> NfsSettingsGlobalSettings {
    self.service = Some(service);
    self
  }

  pub fn service(&self) -> Option<&bool> {
    self.service.as_ref()
  }

  pub fn reset_service(&mut self) {
    self.service = None;
  }

}


