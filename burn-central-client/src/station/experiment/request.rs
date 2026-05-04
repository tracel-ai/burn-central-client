use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperimentRequest {
    pub description: Option<String>,
    pub routine_run: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListExperimentsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummaryQuery {
    pub metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregatedQuery {
    pub metric: String,
    pub max_points: i64,
    pub downsampling_factor: i64,
}
