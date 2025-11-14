use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct CreateExperimentSchema {
    pub description: Option<String>,
    pub code_version_digest: String,
    pub routine_run: String,
}
