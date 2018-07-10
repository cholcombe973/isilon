#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRuleSchedule {
    /// Start time (inclusive) for this schedule, during its specified days.  Format is \"hh:mm\" (24h format hour, and minute).  A null value indicates the beginning of the day (\"00:00\").
    #[serde(rename = "begin")]
    pub begin: Option<String>,
    /// End time (inclusive) for this schedule, during its specified days.  Format is \"hh:mm\" (three-letter weekday name abbreviation, 24h format hour, and minute).  A null value indicates the end of the day (\"23:59\").
    #[serde(rename = "end")]
    pub end: Option<String>,
    /// If true, this rule is in effect on Friday.  If false, or unspecified, it is not.
    #[serde(rename = "friday")]
    pub friday: Option<bool>,
    /// If true, this rule is in effect on Monday.  If false, or unspecified, it is not.
    #[serde(rename = "monday")]
    pub monday: Option<bool>,
    /// If true, this rule is in effect on Saturday.  If false, or unspecified, it is not.
    #[serde(rename = "saturday")]
    pub saturday: Option<bool>,
    /// If true, this rule is in effect on Sunday.  If false, or unspecified, it is not.
    #[serde(rename = "sunday")]
    pub sunday: Option<bool>,
    /// If true, this rule is in effect on Thursday.  If false, or unspecified, it is not.
    #[serde(rename = "thursday")]
    pub thursday: Option<bool>,
    /// If true, this rule is in effect on Tuesday.  If false, or unspecified, it is not.
    #[serde(rename = "tuesday")]
    pub tuesday: Option<bool>,
    /// If true, this rule is in effect on Wednesday.  If false, or unspecified, it is not.
    #[serde(rename = "wednesday")]
    pub wednesday: Option<bool>,
}
