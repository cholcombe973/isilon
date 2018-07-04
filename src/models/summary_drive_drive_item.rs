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
pub struct SummaryDriveDriveItem {
  /// The average operation latency.
  #[serde(rename = "access_latency")]
  access_latency: f32,
  /// The rate of slow (long-latency) operations.
  #[serde(rename = "access_slow")]
  access_slow: f32,
  /// The percentage of time the drive was busy.
  #[serde(rename = "busy")]
  busy: f32,
  /// The rate of bytes written.
  #[serde(rename = "bytes_in")]
  bytes_in: f32,
  /// The rate of bytes read.
  #[serde(rename = "bytes_out")]
  bytes_out: f32,
  /// Drive ID LNN:bay.
  #[serde(rename = "drive_id")]
  drive_id: String,
  /// The average time spent in the I/O scheduler.
  #[serde(rename = "iosched_latency")]
  iosched_latency: f32,
  /// The average length of the I/O scheduler queue.
  #[serde(rename = "iosched_queue")]
  iosched_queue: f32,
  /// Unix Epoch time in seconds of the request.
  #[serde(rename = "time")]
  time: i32,
  /// The type of the drive.
  #[serde(rename = "type")]
  _type: String,
  /// The percent of blocks used on the drive.
  #[serde(rename = "used_bytes_percent")]
  used_bytes_percent: f32,
  /// The number of inodes allocated on the drive.
  #[serde(rename = "used_inodes")]
  used_inodes: f32,
  /// The average size of write operations.
  #[serde(rename = "xfer_size_in")]
  xfer_size_in: f32,
  /// The average size of read operations.
  #[serde(rename = "xfer_size_out")]
  xfer_size_out: f32,
  /// The rate of write operations.
  #[serde(rename = "xfers_in")]
  xfers_in: f32,
  /// The rate of read operations.
  #[serde(rename = "xfers_out")]
  xfers_out: f32
}

impl SummaryDriveDriveItem {
  pub fn new(access_latency: f32, access_slow: f32, busy: f32, bytes_in: f32, bytes_out: f32, drive_id: String, iosched_latency: f32, iosched_queue: f32, time: i32, _type: String, used_bytes_percent: f32, used_inodes: f32, xfer_size_in: f32, xfer_size_out: f32, xfers_in: f32, xfers_out: f32) -> SummaryDriveDriveItem {
    SummaryDriveDriveItem {
      access_latency: access_latency,
      access_slow: access_slow,
      busy: busy,
      bytes_in: bytes_in,
      bytes_out: bytes_out,
      drive_id: drive_id,
      iosched_latency: iosched_latency,
      iosched_queue: iosched_queue,
      time: time,
      _type: _type,
      used_bytes_percent: used_bytes_percent,
      used_inodes: used_inodes,
      xfer_size_in: xfer_size_in,
      xfer_size_out: xfer_size_out,
      xfers_in: xfers_in,
      xfers_out: xfers_out
    }
  }

  pub fn set_access_latency(&mut self, access_latency: f32) {
    self.access_latency = access_latency;
  }

  pub fn with_access_latency(mut self, access_latency: f32) -> SummaryDriveDriveItem {
    self.access_latency = access_latency;
    self
  }

  pub fn access_latency(&self) -> &f32 {
    &self.access_latency
  }


  pub fn set_access_slow(&mut self, access_slow: f32) {
    self.access_slow = access_slow;
  }

  pub fn with_access_slow(mut self, access_slow: f32) -> SummaryDriveDriveItem {
    self.access_slow = access_slow;
    self
  }

  pub fn access_slow(&self) -> &f32 {
    &self.access_slow
  }


  pub fn set_busy(&mut self, busy: f32) {
    self.busy = busy;
  }

  pub fn with_busy(mut self, busy: f32) -> SummaryDriveDriveItem {
    self.busy = busy;
    self
  }

  pub fn busy(&self) -> &f32 {
    &self.busy
  }


  pub fn set_bytes_in(&mut self, bytes_in: f32) {
    self.bytes_in = bytes_in;
  }

  pub fn with_bytes_in(mut self, bytes_in: f32) -> SummaryDriveDriveItem {
    self.bytes_in = bytes_in;
    self
  }

  pub fn bytes_in(&self) -> &f32 {
    &self.bytes_in
  }


  pub fn set_bytes_out(&mut self, bytes_out: f32) {
    self.bytes_out = bytes_out;
  }

  pub fn with_bytes_out(mut self, bytes_out: f32) -> SummaryDriveDriveItem {
    self.bytes_out = bytes_out;
    self
  }

  pub fn bytes_out(&self) -> &f32 {
    &self.bytes_out
  }


  pub fn set_drive_id(&mut self, drive_id: String) {
    self.drive_id = drive_id;
  }

  pub fn with_drive_id(mut self, drive_id: String) -> SummaryDriveDriveItem {
    self.drive_id = drive_id;
    self
  }

  pub fn drive_id(&self) -> &String {
    &self.drive_id
  }


  pub fn set_iosched_latency(&mut self, iosched_latency: f32) {
    self.iosched_latency = iosched_latency;
  }

  pub fn with_iosched_latency(mut self, iosched_latency: f32) -> SummaryDriveDriveItem {
    self.iosched_latency = iosched_latency;
    self
  }

  pub fn iosched_latency(&self) -> &f32 {
    &self.iosched_latency
  }


  pub fn set_iosched_queue(&mut self, iosched_queue: f32) {
    self.iosched_queue = iosched_queue;
  }

  pub fn with_iosched_queue(mut self, iosched_queue: f32) -> SummaryDriveDriveItem {
    self.iosched_queue = iosched_queue;
    self
  }

  pub fn iosched_queue(&self) -> &f32 {
    &self.iosched_queue
  }


  pub fn set_time(&mut self, time: i32) {
    self.time = time;
  }

  pub fn with_time(mut self, time: i32) -> SummaryDriveDriveItem {
    self.time = time;
    self
  }

  pub fn time(&self) -> &i32 {
    &self.time
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> SummaryDriveDriveItem {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_used_bytes_percent(&mut self, used_bytes_percent: f32) {
    self.used_bytes_percent = used_bytes_percent;
  }

  pub fn with_used_bytes_percent(mut self, used_bytes_percent: f32) -> SummaryDriveDriveItem {
    self.used_bytes_percent = used_bytes_percent;
    self
  }

  pub fn used_bytes_percent(&self) -> &f32 {
    &self.used_bytes_percent
  }


  pub fn set_used_inodes(&mut self, used_inodes: f32) {
    self.used_inodes = used_inodes;
  }

  pub fn with_used_inodes(mut self, used_inodes: f32) -> SummaryDriveDriveItem {
    self.used_inodes = used_inodes;
    self
  }

  pub fn used_inodes(&self) -> &f32 {
    &self.used_inodes
  }


  pub fn set_xfer_size_in(&mut self, xfer_size_in: f32) {
    self.xfer_size_in = xfer_size_in;
  }

  pub fn with_xfer_size_in(mut self, xfer_size_in: f32) -> SummaryDriveDriveItem {
    self.xfer_size_in = xfer_size_in;
    self
  }

  pub fn xfer_size_in(&self) -> &f32 {
    &self.xfer_size_in
  }


  pub fn set_xfer_size_out(&mut self, xfer_size_out: f32) {
    self.xfer_size_out = xfer_size_out;
  }

  pub fn with_xfer_size_out(mut self, xfer_size_out: f32) -> SummaryDriveDriveItem {
    self.xfer_size_out = xfer_size_out;
    self
  }

  pub fn xfer_size_out(&self) -> &f32 {
    &self.xfer_size_out
  }


  pub fn set_xfers_in(&mut self, xfers_in: f32) {
    self.xfers_in = xfers_in;
  }

  pub fn with_xfers_in(mut self, xfers_in: f32) -> SummaryDriveDriveItem {
    self.xfers_in = xfers_in;
    self
  }

  pub fn xfers_in(&self) -> &f32 {
    &self.xfers_in
  }


  pub fn set_xfers_out(&mut self, xfers_out: f32) {
    self.xfers_out = xfers_out;
  }

  pub fn with_xfers_out(mut self, xfers_out: f32) -> SummaryDriveDriveItem {
    self.xfers_out = xfers_out;
    self
  }

  pub fn xfers_out(&self) -> &f32 {
    &self.xfers_out
  }


}


