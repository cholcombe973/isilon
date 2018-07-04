/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AntivirusQuarantine : The quarantine status of a file in /ifs.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusQuarantine {
  /// Path of this file, starting with /ifs.
  #[serde(rename = "file")]
  file: String,
  #[serde(rename = "last_istag")]
  last_istag: Option<String>,
  #[serde(rename = "last_scan")]
  last_scan: Option<i32>,
  /// If true, this file is quarantined.  If false, the file is not quarantined.
  #[serde(rename = "quarantined")]
  quarantined: bool,
  /// The result of the last scan on this file.  This string is usually one of: never_scanned, clean, quarantined, repaired, truncated, infected_no_action_taken, skipped_per_settings.  However, a longer string starting with 'unknown_status' and describing the details can also appear in uncommon edge cases.
  #[serde(rename = "scan_result")]
  scan_result: String,
  /// The scanning status of this file.  If 'current', the file was scanned with the most up-to-date virus definitions.  If 'not_current', it has either not been scanned, been modified since the last scan, or the virus definitions are not current.
  #[serde(rename = "scan_status")]
  scan_status: String
}

impl AntivirusQuarantine {
  /// The quarantine status of a file in /ifs.
  pub fn new(file: String, quarantined: bool, scan_result: String, scan_status: String) -> AntivirusQuarantine {
    AntivirusQuarantine {
      file: file,
      last_istag: None,
      last_scan: None,
      quarantined: quarantined,
      scan_result: scan_result,
      scan_status: scan_status
    }
  }

  pub fn set_file(&mut self, file: String) {
    self.file = file;
  }

  pub fn with_file(mut self, file: String) -> AntivirusQuarantine {
    self.file = file;
    self
  }

  pub fn file(&self) -> &String {
    &self.file
  }


  pub fn set_last_istag(&mut self, last_istag: String) {
    self.last_istag = Some(last_istag);
  }

  pub fn with_last_istag(mut self, last_istag: String) -> AntivirusQuarantine {
    self.last_istag = Some(last_istag);
    self
  }

  pub fn last_istag(&self) -> Option<&String> {
    self.last_istag.as_ref()
  }

  pub fn reset_last_istag(&mut self) {
    self.last_istag = None;
  }

  pub fn set_last_scan(&mut self, last_scan: i32) {
    self.last_scan = Some(last_scan);
  }

  pub fn with_last_scan(mut self, last_scan: i32) -> AntivirusQuarantine {
    self.last_scan = Some(last_scan);
    self
  }

  pub fn last_scan(&self) -> Option<&i32> {
    self.last_scan.as_ref()
  }

  pub fn reset_last_scan(&mut self) {
    self.last_scan = None;
  }

  pub fn set_quarantined(&mut self, quarantined: bool) {
    self.quarantined = quarantined;
  }

  pub fn with_quarantined(mut self, quarantined: bool) -> AntivirusQuarantine {
    self.quarantined = quarantined;
    self
  }

  pub fn quarantined(&self) -> &bool {
    &self.quarantined
  }


  pub fn set_scan_result(&mut self, scan_result: String) {
    self.scan_result = scan_result;
  }

  pub fn with_scan_result(mut self, scan_result: String) -> AntivirusQuarantine {
    self.scan_result = scan_result;
    self
  }

  pub fn scan_result(&self) -> &String {
    &self.scan_result
  }


  pub fn set_scan_status(&mut self, scan_status: String) {
    self.scan_status = scan_status;
  }

  pub fn with_scan_status(mut self, scan_status: String) -> AntivirusQuarantine {
    self.scan_status = scan_status;
    self
  }

  pub fn scan_status(&self) -> &String {
    &self.scan_status
  }


}


