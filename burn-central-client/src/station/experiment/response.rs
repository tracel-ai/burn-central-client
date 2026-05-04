use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct ExperimentResponse {
    pub id: i32,
    pub experiment_num: i32,
    pub status: String,
    pub description: String,
    pub created_at: String,
    pub training_function: Option<TrainingFunctionResponse>,
    pub arguments: Value,
    pub inputs: Vec<ExperimentInputResponse>,
    pub configurations: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExperimentInputResponse {
    Artifact { artifact_id: String },
    Model { model_version_id: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrainingFunctionResponse {
    pub mod_path: String,
    pub fn_name: String,
    pub proc_type: String,
    pub code: String,
    pub routine: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExperimentListResponse {
    pub items: Vec<ExperimentResponse>,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricMetadataResponse {
    pub metric_types: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricSummaryGroupResponse {
    pub group: String,
    pub optimal_value: f64,
    pub epoch: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricSummaryResponse {
    pub groups: Vec<MetricSummaryGroupResponse>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricEntryResponse {
    pub epoch: usize,
    pub iteration: usize,
    pub value: f64,
    pub low: f64,
    pub high: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricGroupResponse {
    pub name: String,
    pub entries: Vec<MetricEntryResponse>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricResponse {
    pub groups: Vec<MetricGroupResponse>,
}
