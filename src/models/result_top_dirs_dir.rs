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
pub struct ResultTopDirsDir {
  /// Directory access time
  #[serde(rename = "atime")]
  atime: i32,
  /// Directory creation begin time.
  #[serde(rename = "btime")]
  btime: i32,
  /// Unix inode change time.
  #[serde(rename = "ctime")]
  ctime: i32,
  /// Relative directory path under /ifs/.
  #[serde(rename = "path")]
  path: String
}

impl ResultTopDirsDir {
  pub fn new(atime: i32, btime: i32, ctime: i32, path: String) -> ResultTopDirsDir {
    ResultTopDirsDir {
      atime: atime,
      btime: btime,
      ctime: ctime,
      path: path
    }
  }

  pub fn set_atime(&mut self, atime: i32) {
    self.atime = atime;
  }

  pub fn with_atime(mut self, atime: i32) -> ResultTopDirsDir {
    self.atime = atime;
    self
  }

  pub fn atime(&self) -> &i32 {
    &self.atime
  }


  pub fn set_btime(&mut self, btime: i32) {
    self.btime = btime;
  }

  pub fn with_btime(mut self, btime: i32) -> ResultTopDirsDir {
    self.btime = btime;
    self
  }

  pub fn btime(&self) -> &i32 {
    &self.btime
  }


  pub fn set_ctime(&mut self, ctime: i32) {
    self.ctime = ctime;
  }

  pub fn with_ctime(mut self, ctime: i32) -> ResultTopDirsDir {
    self.ctime = ctime;
    self
  }

  pub fn ctime(&self) -> &i32 {
    &self.ctime
  }


  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> ResultTopDirsDir {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


}


