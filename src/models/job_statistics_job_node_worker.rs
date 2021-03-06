#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeWorker {
    /// The sleep-to-work ratio of this worker; how much time it spends sleeping compared to working.
    #[serde(rename = "stw")]
    pub stw: Option<f32>,
    /// A representation of the task the worker is currently processing; not intended to be read by humans.
    #[serde(rename = "task")]
    pub task: Option<String>,
    /// A representation of the most recent task result produced by the worker; not intended to be read by humans.
    #[serde(rename = "task_result")]
    pub task_result: Option<String>,
    /// The worker ID.
    #[serde(rename = "worker")]
    pub worker: i32,
}
