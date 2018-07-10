

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRuleSchedule {
  /// Start time (inclusive) for this schedule, during its specified days.  Format is \"hh:mm\" (24h format hour, and minute).  A null value indicates the beginning of the day (\"00:00\").
  #[serde(rename = "begin")]
  begin: Option<String>,
  /// End time (inclusive) for this schedule, during its specified days.  Format is \"hh:mm\" (three-letter weekday name abbreviation, 24h format hour, and minute).  A null value indicates the end of the day (\"23:59\").
  #[serde(rename = "end")]
  end: Option<String>,
  /// If true, this rule is in effect on Friday.  If false, or unspecified, it is not.
  #[serde(rename = "friday")]
  friday: Option<bool>,
  /// If true, this rule is in effect on Monday.  If false, or unspecified, it is not.
  #[serde(rename = "monday")]
  monday: Option<bool>,
  /// If true, this rule is in effect on Saturday.  If false, or unspecified, it is not.
  #[serde(rename = "saturday")]
  saturday: Option<bool>,
  /// If true, this rule is in effect on Sunday.  If false, or unspecified, it is not.
  #[serde(rename = "sunday")]
  sunday: Option<bool>,
  /// If true, this rule is in effect on Thursday.  If false, or unspecified, it is not.
  #[serde(rename = "thursday")]
  thursday: Option<bool>,
  /// If true, this rule is in effect on Tuesday.  If false, or unspecified, it is not.
  #[serde(rename = "tuesday")]
  tuesday: Option<bool>,
  /// If true, this rule is in effect on Wednesday.  If false, or unspecified, it is not.
  #[serde(rename = "wednesday")]
  wednesday: Option<bool>
}

