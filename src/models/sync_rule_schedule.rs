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

impl SyncRuleSchedule {
  pub fn new() -> SyncRuleSchedule {
    SyncRuleSchedule {
      begin: None,
      end: None,
      friday: None,
      monday: None,
      saturday: None,
      sunday: None,
      thursday: None,
      tuesday: None,
      wednesday: None
    }
  }

  pub fn set_begin(&mut self, begin: String) {
    self.begin = Some(begin);
  }

  pub fn with_begin(mut self, begin: String) -> SyncRuleSchedule {
    self.begin = Some(begin);
    self
  }

  pub fn begin(&self) -> Option<&String> {
    self.begin.as_ref()
  }

  pub fn reset_begin(&mut self) {
    self.begin = None;
  }

  pub fn set_end(&mut self, end: String) {
    self.end = Some(end);
  }

  pub fn with_end(mut self, end: String) -> SyncRuleSchedule {
    self.end = Some(end);
    self
  }

  pub fn end(&self) -> Option<&String> {
    self.end.as_ref()
  }

  pub fn reset_end(&mut self) {
    self.end = None;
  }

  pub fn set_friday(&mut self, friday: bool) {
    self.friday = Some(friday);
  }

  pub fn with_friday(mut self, friday: bool) -> SyncRuleSchedule {
    self.friday = Some(friday);
    self
  }

  pub fn friday(&self) -> Option<&bool> {
    self.friday.as_ref()
  }

  pub fn reset_friday(&mut self) {
    self.friday = None;
  }

  pub fn set_monday(&mut self, monday: bool) {
    self.monday = Some(monday);
  }

  pub fn with_monday(mut self, monday: bool) -> SyncRuleSchedule {
    self.monday = Some(monday);
    self
  }

  pub fn monday(&self) -> Option<&bool> {
    self.monday.as_ref()
  }

  pub fn reset_monday(&mut self) {
    self.monday = None;
  }

  pub fn set_saturday(&mut self, saturday: bool) {
    self.saturday = Some(saturday);
  }

  pub fn with_saturday(mut self, saturday: bool) -> SyncRuleSchedule {
    self.saturday = Some(saturday);
    self
  }

  pub fn saturday(&self) -> Option<&bool> {
    self.saturday.as_ref()
  }

  pub fn reset_saturday(&mut self) {
    self.saturday = None;
  }

  pub fn set_sunday(&mut self, sunday: bool) {
    self.sunday = Some(sunday);
  }

  pub fn with_sunday(mut self, sunday: bool) -> SyncRuleSchedule {
    self.sunday = Some(sunday);
    self
  }

  pub fn sunday(&self) -> Option<&bool> {
    self.sunday.as_ref()
  }

  pub fn reset_sunday(&mut self) {
    self.sunday = None;
  }

  pub fn set_thursday(&mut self, thursday: bool) {
    self.thursday = Some(thursday);
  }

  pub fn with_thursday(mut self, thursday: bool) -> SyncRuleSchedule {
    self.thursday = Some(thursday);
    self
  }

  pub fn thursday(&self) -> Option<&bool> {
    self.thursday.as_ref()
  }

  pub fn reset_thursday(&mut self) {
    self.thursday = None;
  }

  pub fn set_tuesday(&mut self, tuesday: bool) {
    self.tuesday = Some(tuesday);
  }

  pub fn with_tuesday(mut self, tuesday: bool) -> SyncRuleSchedule {
    self.tuesday = Some(tuesday);
    self
  }

  pub fn tuesday(&self) -> Option<&bool> {
    self.tuesday.as_ref()
  }

  pub fn reset_tuesday(&mut self) {
    self.tuesday = None;
  }

  pub fn set_wednesday(&mut self, wednesday: bool) {
    self.wednesday = Some(wednesday);
  }

  pub fn with_wednesday(mut self, wednesday: bool) -> SyncRuleSchedule {
    self.wednesday = Some(wednesday);
    self
  }

  pub fn wednesday(&self) -> Option<&bool> {
    self.wednesday.as_ref()
  }

  pub fn reset_wednesday(&mut self) {
    self.wednesday = None;
  }

}


