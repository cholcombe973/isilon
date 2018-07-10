

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettings {
  /// The default filepool policy values for cloudpools.
  #[serde(rename = "cloud_policy_defaults")]
  cloud_policy_defaults: Option<::models::CloudSettingsSettingsCloudPolicyDefaults>,
  /// Coefficients in the quadratic function for determining the rest period between successive archive attempts.
  #[serde(rename = "retry_coefficient_archive")]
  retry_coefficient_archive: Option<String>,
  /// Coefficients in the quadratic function for determining the rest period between successive cache invalidation attempts.
  #[serde(rename = "retry_coefficient_cache_invalidation")]
  retry_coefficient_cache_invalidation: Option<String>,
  /// Coefficients in the quadratic function for determining the rest period between successive cloud garbage collection attempts.
  #[serde(rename = "retry_coefficient_cloud_garbage_collection")]
  retry_coefficient_cloud_garbage_collection: Option<String>,
  /// Coefficients in the quadratic function for determining the rest period between successive local garbage collection attempts.
  #[serde(rename = "retry_coefficient_local_garbage_collection")]
  retry_coefficient_local_garbage_collection: Option<String>,
  /// Coefficients in the quadratic function for determining the rest period between successive read ahead attempts.
  #[serde(rename = "retry_coefficient_read_ahead")]
  retry_coefficient_read_ahead: Option<String>,
  /// Coefficients in the quadratic function for determining the rest period between successive recall attempts.
  #[serde(rename = "retry_coefficient_recall")]
  retry_coefficient_recall: Option<String>,
  /// Coefficients in the quadratic function for determining the rest period between successive writeback attempts.
  #[serde(rename = "retry_coefficient_writeback")]
  retry_coefficient_writeback: Option<String>,
  /// Amount of time to wait between successive file archive operations.
  #[serde(rename = "sleep_timeout_archive")]
  sleep_timeout_archive: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
  /// Amount of time to wait between successive file cache_invalidation operations.
  #[serde(rename = "sleep_timeout_cache_invalidation")]
  sleep_timeout_cache_invalidation: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
  /// Amount of time to wait between successive file cloud garbage collection operations.
  #[serde(rename = "sleep_timeout_cloud_garbage_collection")]
  sleep_timeout_cloud_garbage_collection: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
  /// Amount of time to wait between successive file local garbage collection operations.
  #[serde(rename = "sleep_timeout_local_garbage_collection")]
  sleep_timeout_local_garbage_collection: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
  /// Amount of time to wait between successive file read ahead operations.
  #[serde(rename = "sleep_timeout_read_ahead")]
  sleep_timeout_read_ahead: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
  /// Amount of time to wait between successive file recall operations.
  #[serde(rename = "sleep_timeout_recall")]
  sleep_timeout_recall: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>,
  /// Amount of time to wait between successive file writeback operations.
  #[serde(rename = "sleep_timeout_writeback")]
  sleep_timeout_writeback: Option<::models::CloudSettingsSettingsSleepTimeoutCloudGarbageCollection>
}

