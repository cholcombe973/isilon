

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHardwareNode {
  /// Name of this node's chassis.
  #[serde(rename = "chassis")]
  chassis: Option<String>,
  /// Chassis code of this node (1U, 2U, etc.).
  #[serde(rename = "chassis_code")]
  chassis_code: Option<String>,
  /// Number of chassis making up this node.
  #[serde(rename = "chassis_count")]
  chassis_count: Option<String>,
  /// Chassis depth for this node if applicable (Normal, Deep, Unknown). If not supported: Unknown. If Logic to determine chassis depth fails: Unknown. If PSI_Get fails: Unknown. PSI_Get can fail if PSI not initialized, or key does not exist.
  #[serde(rename = "chassis_depth")]
  chassis_depth: Option<String>,
  /// Class of this node (storage, accelerator, etc.).
  #[serde(rename = "class")]
  class: Option<String>,
  /// Code name of this node if applicable (Minnetonka, MiniHuron, Huron, Union, Tahoe, Superior, Unknown). If not supported: Unknown. If Logic to determine code name fails: Unknown. If PSI_Get fails: Unknown. PSI_Get can fail if PSI not initialized, or key does not exist.
  #[serde(rename = "code_name")]
  code_name: Option<String>,
  /// Type of compute node if applicable (Low, Medium, High, Turbo, Ultra, Unknown). If not supported: Unknown. If Logic to determine compute type fails: Unknown. If PSI_Get fails: Unknown. PSI_Get can fail if PSI not initialized, or key does not exist.
  #[serde(rename = "compute_type")]
  compute_type: Option<String>,
  /// Node configuration ID.
  #[serde(rename = "configuration_id")]
  configuration_id: Option<String>,
  /// Manufacturer and model of this node's CPU.
  #[serde(rename = "cpu")]
  cpu: Option<String>,
  /// Manufacturer and model of this node's disk controller.
  #[serde(rename = "disk_controller")]
  disk_controller: Option<String>,
  /// Manufacturer and model of this node's disk expander.
  #[serde(rename = "disk_expander")]
  disk_expander: Option<String>,
  /// Family code of this node (X, S, NL, etc.).
  #[serde(rename = "family_code")]
  family_code: Option<String>,
  /// Manufacturer, model, and device id of this node's flash drive.
  #[serde(rename = "flash_drive")]
  flash_drive: Option<String>,
  /// Generation code of this node.
  #[serde(rename = "generation_code")]
  generation_code: Option<String>,
  /// Isilon hardware generation name.
  #[serde(rename = "hwgen")]
  hwgen: Option<String>,
  /// Node ID (Device Number) of this node.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Version of this node's Isilon Management Board.
  #[serde(rename = "imb_version")]
  imb_version: Option<String>,
  /// Infiniband card type.
  #[serde(rename = "infiniband")]
  infiniband: Option<String>,
  /// Version of the LCD panel.
  #[serde(rename = "lcd_version")]
  lcd_version: Option<String>,
  /// Logical Node Number (LNN) of this node.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Isilon node model identifier string (S200, X410, Infinity-H500, etc.).
  #[serde(rename = "model")]
  model: Option<String>,
  /// Isilon node model code string (S200, X410, H500, etc.).
  #[serde(rename = "model_code")]
  model_code: Option<String>,
  /// Manufacturer and model of this node's motherboard.
  #[serde(rename = "motherboard")]
  motherboard: Option<String>,
  /// Description of all this node's network interfaces.
  #[serde(rename = "net_interfaces")]
  net_interfaces: Option<String>,
  /// Position of node within chassis (e.g., 1-4 for Infinity chassis). -1 for error or not supported.
  #[serde(rename = "node_slot_id")]
  node_slot_id: Option<i32>,
  /// Manufacturer and model of this node's NVRAM board.
  #[serde(rename = "nvram")]
  nvram: Option<String>,
  /// Serial number of this node's peer/buddy node.(Infinity Only)
  #[serde(rename = "peer_serial_number")]
  peer_serial_number: Option<String>,
  /// Performance code of this node, if applicable (2, 4, 5, etc.).
  #[serde(rename = "performance_code")]
  performance_code: Option<String>,
  /// Description strings for each power supply on this node.
  #[serde(rename = "powersupplies")]
  powersupplies: Option<Vec<String>>,
  /// Number of processors and cores on this node.
  #[serde(rename = "processor")]
  processor: Option<String>,
  /// Isilon product name.
  #[serde(rename = "product")]
  product: Option<String>,
  /// Size of RAM in bytes.
  #[serde(rename = "ram")]
  ram: Option<i32>,
  /// Serial number of this node.
  #[serde(rename = "serial_number")]
  serial_number: Option<String>,
  /// Series of this node (X, I, NL, etc.).
  #[serde(rename = "series")]
  series: Option<String>,
  /// Size of drive sleds in node, if applicable. Expected values: 3, 4, 6. 0 if unable to determine sled size. -1 for error or not supported. If PSI_Get fails: -1. PSI_Get can fail if PSI not initialized, or key does not exist.
  #[serde(rename = "sled_drive_count")]
  sled_drive_count: Option<i32>,
  /// Storage class of this node (storage or diskless).
  #[serde(rename = "storage_class")]
  storage_class: Option<String>,
  /// Platform tier level of this node if applicable (1-4 are defined, 0 for unknown or not supported, -1 for error). If not supported: 0. If Logic to determine tier fails: 0 for unknown. If PSI_Get fails: -1 for error. PSI_Get can fail if PSI not initialized, or key does not exist.
  #[serde(rename = "tier")]
  tier: Option<i32>,
  /// Serial number of the top level assembly of this node.(Infinity Only)
  #[serde(rename = "top_level_assembly_serial_number")]
  top_level_assembly_serial_number: String
}

