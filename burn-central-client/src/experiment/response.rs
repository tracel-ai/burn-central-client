use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ExperimentResponse {
    pub experiment_num: i32,
}
