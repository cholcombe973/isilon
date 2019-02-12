#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettings {
    /// The default filepool policy values for cloudpools.
    #[serde(rename = "cloud_policy_defaults")]
    pub cloud_policy_defaults: Option <crate::models::CloudSettingsSettingsCloudPolicyDefaults>,
    /// Coefficients in the quadratic function for determining the rest period between successive archive attempts.
    #[serde(rename = "retry_coefficient_archive")]
    pub retry_coefficient_archive: Option<String>,
    /// Coefficients in the quadratic function for determining the rest period between successive cache invalidation attempts.
    #[serde(rename = "retry_coefficient_cache_invalidation")]
    pub retry_coefficient_cache_invalidation: Option<String>,
    /// Coefficients in the quadratic function for determining the rest period between successive cloud garbage collection attempts.
    #[serde(rename = "retry_coefficient_cloud_garbage_collection")]
    pub retry_coefficient_cloud_garbage_collection: Option<String>,
    /// Coefficients in the quadratic function for determining the rest period between successive local garbage collection attempts.
    #[serde(rename = "retry_coefficient_local_garbage_collection")]
    pub retry_coefficient_local_garbage_collection: Option<String>,
    /// Coefficients in the quadratic function for determining the rest period between successive read ahead attempts.
    #[serde(rename = "retry_coefficient_read_ahead")]
    pub retry_coefficient_read_ahead: Option<String>,
    /// Coefficients in the quadratic function for determining the rest period between successive recall attempts.
    #[serde(rename = "retry_coefficient_recall")]
    pub retry_coefficient_recall: Option<String>,
    /// Coefficients in the quadratic function for determining the rest period between successive writeback attempts.
    #[serde(rename = "retry_coefficient_writeback")]
    pub retry_coefficient_writeback: Option<String>,
    /// Amount of time to wait between successive file archive operations.
    #[serde(rename = "sleep_timeout_archive")]
    pub sleep_timeout_archive:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
    /// Amount of time to wait between successive file cache_invalidation operations.
    #[serde(rename = "sleep_timeout_cache_invalidation")]
    pub sleep_timeout_cache_invalidation:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
    /// Amount of time to wait between successive file cloud garbage collection operations.
    #[serde(rename = "sleep_timeout_cloud_garbage_collection")]
    pub sleep_timeout_cloud_garbage_collection:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
    /// Amount of time to wait between successive file local garbage collection operations.
    #[serde(rename = "sleep_timeout_local_garbage_collection")]
    pub sleep_timeout_local_garbage_collection:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
    /// Amount of time to wait between successive file read ahead operations.
    #[serde(rename = "sleep_timeout_read_ahead")]
    pub sleep_timeout_read_ahead:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
    /// Amount of time to wait between successive file recall operations.
    #[serde(rename = "sleep_timeout_recall")]
    pub sleep_timeout_recall:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
    /// Amount of time to wait between successive file writeback operations.
    #[serde(rename = "sleep_timeout_writeback")]
    pub sleep_timeout_writeback:
        Option <crate::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
}
