
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

