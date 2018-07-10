

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolSettingsSettings {
  /// Automatically manage IO optimization settings on files.
  #[serde(rename = "automatically_manage_io_optimization")]
  automatically_manage_io_optimization: String,
  /// Automatically manage protection settings on files.
  #[serde(rename = "automatically_manage_protection")]
  automatically_manage_protection: String,
  /// Optimize namespace operations by storing metadata on SSDs.
  #[serde(rename = "global_namespace_acceleration_enabled")]
  global_namespace_acceleration_enabled: bool,
  /// Whether or not namespace operation optimizations are currently in effect.
  #[serde(rename = "global_namespace_acceleration_state")]
  global_namespace_acceleration_state: String,
  /// Automatically add additional protection level to all directories.
  #[serde(rename = "protect_directories_one_level_higher")]
  protect_directories_one_level_higher: bool,
  /// Spill writes into other pools as needed.
  #[serde(rename = "spillover_enabled")]
  spillover_enabled: bool,
  /// Target pool for spilled writes.
  #[serde(rename = "spillover_target")]
  spillover_target: ::models::StoragepoolSettingsSettingsSpilloverTarget,
  /// The L3 Cache default enabled state. This specifies whether L3 Cache should be enabled on new node pools.
  #[serde(rename = "ssd_l3_cache_default_enabled")]
  ssd_l3_cache_default_enabled: bool,
  /// Controls number of mirrors of QAB blocks to place on SSDs.
  #[serde(rename = "ssd_qab_mirrors")]
  ssd_qab_mirrors: String,
  /// Controls number of mirrors of system B-tree blocks to place on SSDs.
  #[serde(rename = "ssd_system_btree_mirrors")]
  ssd_system_btree_mirrors: String,
  /// Controls number of mirrors of system delta blocks to place on SSDs.
  #[serde(rename = "ssd_system_delta_mirrors")]
  ssd_system_delta_mirrors: String,
  /// Deny writes into reserved virtual hot spare space.
  #[serde(rename = "virtual_hot_spare_deny_writes")]
  virtual_hot_spare_deny_writes: bool,
  /// Hide reserved virtual hot spare space from free space counts.
  #[serde(rename = "virtual_hot_spare_hide_spare")]
  virtual_hot_spare_hide_spare: bool,
  /// The number of drives to reserve for the virtual hot spare, from 0-4.
  #[serde(rename = "virtual_hot_spare_limit_drives")]
  virtual_hot_spare_limit_drives: i32,
  /// The percent space to reserve for the virtual hot spare, from 0-20.
  #[serde(rename = "virtual_hot_spare_limit_percent")]
  virtual_hot_spare_limit_percent: i32
}

