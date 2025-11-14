use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct CreateExperimentSchema {
    pub description: Option<String>,
    pub code_version_digest: String,
    pub routine_run: String,
}
