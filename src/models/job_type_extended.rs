#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobTypeExtended {
    /// Whether the job type is enabled and able to run.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Default impact policy of this job type.
    #[serde(rename = "policy")]
    pub policy: String,
    /// Default priority of this job type; lower numbers preempt higher numbers.
    #[serde(rename = "priority")]
    pub priority: i32,
    /// The schedule on which this job type is queued, if any.
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
    /// Whether multiple instances of this job type may run simultaneously.
    #[serde(rename = "allow_multiple_instances")]
    pub allow_multiple_instances: bool,
    /// Brief description of the job type.
    #[serde(rename = "description")]
    pub description: String,
    /// The set(s) of mutually-exclusive job types to which this job belongs.  No job in this set may run with any other job in this set.  Obsolete; this value will always be an empty string, as exclusion sets are no longer a job type property.
    #[serde(rename = "exclusion_set")]
    pub exclusion_set: String,
    /// Whether this job type is normally visible in the UI.
    #[serde(rename = "hidden")]
    pub hidden: bool,
    /// Job type ID.
    #[serde(rename = "id")]
    pub id: String,
}
