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
pub struct JobStatisticsJobNodeWorker {
  /// The sleep-to-work ratio of this worker; how much time it spends sleeping compared to working.
  #[serde(rename = "stw")]
  stw: Option<f32>,
  /// A representation of the task the worker is currently processing; not intended to be read by humans.
  #[serde(rename = "task")]
  task: Option<String>,
  /// A representation of the most recent task result produced by the worker; not intended to be read by humans.
  #[serde(rename = "task_result")]
  task_result: Option<String>,
  /// The worker ID.
  #[serde(rename = "worker")]
  worker: i32
}

impl JobStatisticsJobNodeWorker {
  pub fn new(worker: i32) -> JobStatisticsJobNodeWorker {
    JobStatisticsJobNodeWorker {
      stw: None,
      task: None,
      task_result: None,
      worker: worker
    }
  }

  pub fn set_stw(&mut self, stw: f32) {
    self.stw = Some(stw);
  }

  pub fn with_stw(mut self, stw: f32) -> JobStatisticsJobNodeWorker {
    self.stw = Some(stw);
    self
  }

  pub fn stw(&self) -> Option<&f32> {
    self.stw.as_ref()
  }

  pub fn reset_stw(&mut self) {
    self.stw = None;
  }

  pub fn set_task(&mut self, task: String) {
    self.task = Some(task);
  }

  pub fn with_task(mut self, task: String) -> JobStatisticsJobNodeWorker {
    self.task = Some(task);
    self
  }

  pub fn task(&self) -> Option<&String> {
    self.task.as_ref()
  }

  pub fn reset_task(&mut self) {
    self.task = None;
  }

  pub fn set_task_result(&mut self, task_result: String) {
    self.task_result = Some(task_result);
  }

  pub fn with_task_result(mut self, task_result: String) -> JobStatisticsJobNodeWorker {
    self.task_result = Some(task_result);
    self
  }

  pub fn task_result(&self) -> Option<&String> {
    self.task_result.as_ref()
  }

  pub fn reset_task_result(&mut self) {
    self.task_result = None;
  }

  pub fn set_worker(&mut self, worker: i32) {
    self.worker = worker;
  }

  pub fn with_worker(mut self, worker: i32) -> JobStatisticsJobNodeWorker {
    self.worker = worker;
    self
  }

  pub fn worker(&self) -> &i32 {
    &self.worker
  }


}


