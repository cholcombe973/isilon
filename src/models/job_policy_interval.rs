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
pub struct JobPolicyInterval {
  /// Beginning time for the corresponding impact, in the format 'WWWW HH:MM', where 'WWWW' is the full English name of the day of the week, 'HH' is the hour (00-23), and 'MM' is the minute (00-59).
  #[serde(rename = "begin")]
  begin: String,
  /// Ending time for the corresponding impact, in the format 'WWWW HH:MM', where 'WWWW' is the full English name of the day of the week, 'HH' is the hour (00-23), and 'MM' is the minute (00-59).
  #[serde(rename = "end")]
  end: String,
  /// Impact for the corresponding time span.
  #[serde(rename = "impact")]
  impact: String
}

impl JobPolicyInterval {
  pub fn new(begin: String, end: String, impact: String) -> JobPolicyInterval {
    JobPolicyInterval {
      begin: begin,
      end: end,
      impact: impact
    }
  }

  pub fn set_begin(&mut self, begin: String) {
    self.begin = begin;
  }

  pub fn with_begin(mut self, begin: String) -> JobPolicyInterval {
    self.begin = begin;
    self
  }

  pub fn begin(&self) -> &String {
    &self.begin
  }


  pub fn set_end(&mut self, end: String) {
    self.end = end;
  }

  pub fn with_end(mut self, end: String) -> JobPolicyInterval {
    self.end = end;
    self
  }

  pub fn end(&self) -> &String {
    &self.end
  }


  pub fn set_impact(&mut self, impact: String) {
    self.impact = impact;
  }

  pub fn with_impact(mut self, impact: String) -> JobPolicyInterval {
    self.impact = impact;
    self
  }

  pub fn impact(&self) -> &String {
    &self.impact
  }


}


