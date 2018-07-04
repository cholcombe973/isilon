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
pub struct NtpServers {
  #[serde(rename = "servers")]
  servers: Option<Vec<::models::NtpServerExtended>>
}

impl NtpServers {
  pub fn new() -> NtpServers {
    NtpServers {
      servers: None
    }
  }

  pub fn set_servers(&mut self, servers: Vec<::models::NtpServerExtended>) {
    self.servers = Some(servers);
  }

  pub fn with_servers(mut self, servers: Vec<::models::NtpServerExtended>) -> NtpServers {
    self.servers = Some(servers);
    self
  }

  pub fn servers(&self) -> Option<&Vec<::models::NtpServerExtended>> {
    self.servers.as_ref()
  }

  pub fn reset_servers(&mut self) {
    self.servers = None;
  }

}


