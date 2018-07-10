

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolSettingsExtended {
  /// Automatically manage IO optimization settings on files.
  #[serde(rename = "automatically_manage_io_optimization")]
  automatically_manage_io_optimization: Option<String>,
  /// Automatically manage protection settings on files.
  #[serde(rename = "automatically_manage_protection")]
  automatically_manage_protection: Option<String>,
  /// Optimize namespace operations by storing metadata on SSDs.
  #[serde(rename = "global_namespace_acceleration_enabled")]
  global_namespace_acceleration_enabled: Option<bool>,
  /// Automatically add additional protection level to all directories.
  #[serde(rename = "protect_directories_one_level_higher")]
  protect_directories_one_level_higher: Option<bool>,
  /// Spill writes into other pools as needed.
  #[serde(rename = "spillover_enabled")]
  spillover_enabled: Option<bool>,
  /// Target pool for spilled writes.
  #[serde(rename = "spillover_target")]
  spillover_target: Option<::models::StoragepoolSettingsSpilloverTarget>,
  /// The L3 Cache default enabled state. This specifies whether L3 Cache should be enabled on new node pools
  #[serde(rename = "ssd_l3_cache_default_enabled")]
  ssd_l3_cache_default_enabled: Option<bool>,
  /// Controls number of mirrors of QAB blocks to place on SSDs.
  #[serde(rename = "ssd_qab_mirrors")]
  ssd_qab_mirrors: Option<String>,
  /// Controls number of mirrors of system B-tree blocks to place on SSDs.
  #[serde(rename = "ssd_system_btree_mirrors")]
  ssd_system_btree_mirrors: Option<String>,
  /// Controls number of mirrors of system delta blocks to place on SSDs.
  #[serde(rename = "ssd_system_delta_mirrors")]
  ssd_system_delta_mirrors: Option<String>,
  /// Deny writes into reserved virtual hot spare space.
  #[serde(rename = "virtual_hot_spare_deny_writes")]
  virtual_hot_spare_deny_writes: Option<bool>,
  /// Hide reserved virtual hot spare space from free space counts.
  #[serde(rename = "virtual_hot_spare_hide_spare")]
  virtual_hot_spare_hide_spare: Option<bool>,
  /// The number of drives to reserve for the virtual hot spare, from 0-4.
  #[serde(rename = "virtual_hot_spare_limit_drives")]
  virtual_hot_spare_limit_drives: Option<i32>,
  /// The percent space to reserve for the virtual hot spare, from 0-20.
  #[serde(rename = "virtual_hot_spare_limit_percent")]
  virtual_hot_spare_limit_percent: Option<i32>
}

