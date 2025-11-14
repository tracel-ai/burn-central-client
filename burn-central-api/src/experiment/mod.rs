pub mod request;
pub mod response;

use crate::{
    Client, ClientError,
    experiment::{request::CreateExperimentSchema, response::ExperimentResponse},
};

impl Client {
    /// Formats a WebSocket URL for the given experiment.
    pub fn format_websocket_url(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
    ) -> String {
        let mut url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/ws"
        ));
        url.set_scheme(if self.base_url.scheme() == "https" {
            "wss"
        } else {
            "ws"
        })
        .expect("Should be able to set ws scheme");

        url.to_string()
    }

    /// Create a new experiment for the given project.
    ///
    /// The client must be logged in before calling this method.
    pub fn create_experiment(
        &self,
        owner_name: &str,
        project_name: &str,
        description: Option<String>,
        code_version_digest: String,
        routine: String,
    ) -> Result<ExperimentResponse, ClientError> {
        let url = self.join(&format!("projects/{owner_name}/{project_name}/experiments"));

        // Create a new experiment
        let experiment_response = self.post_json::<CreateExperimentSchema, ExperimentResponse>(
            url,
            Some(CreateExperimentSchema {
                description,
                code_version_digest,
                routine_run: routine,
            }),
        )?;

        Ok(experiment_response)
    }
}
