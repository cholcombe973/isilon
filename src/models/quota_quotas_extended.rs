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
pub struct QuotaQuotasExtended {
  #[serde(rename = "quotas")]
  quotas: Option<Vec<::models::QuotaQuotaExtended>>,
  /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
  #[serde(rename = "resume")]
  resume: Option<String>
}

impl QuotaQuotasExtended {
  pub fn new() -> QuotaQuotasExtended {
    QuotaQuotasExtended {
      quotas: None,
      resume: None
    }
  }

  pub fn set_quotas(&mut self, quotas: Vec<::models::QuotaQuotaExtended>) {
    self.quotas = Some(quotas);
  }

  pub fn with_quotas(mut self, quotas: Vec<::models::QuotaQuotaExtended>) -> QuotaQuotasExtended {
    self.quotas = Some(quotas);
    self
  }

  pub fn quotas(&self) -> Option<&Vec<::models::QuotaQuotaExtended>> {
    self.quotas.as_ref()
  }

  pub fn reset_quotas(&mut self) {
    self.quotas = None;
  }

  pub fn set_resume(&mut self, resume: String) {
    self.resume = Some(resume);
  }

  pub fn with_resume(mut self, resume: String) -> QuotaQuotasExtended {
    self.resume = Some(resume);
    self
  }

  pub fn resume(&self) -> Option<&String> {
    self.resume.as_ref()
  }

  pub fn reset_resume(&mut self) {
    self.resume = None;
  }

}


