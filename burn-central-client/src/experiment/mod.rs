pub mod request;
pub mod response;
pub mod websocket;

use crate::{
    Client, ClientError, WebSocketClient,
    experiment::{request::CreateExperimentSchema, response::ExperimentResponse},
    websocket::WebSocketError,
};

impl Client {
    /// Formats a WebSocket URL for the given experiment.
    fn format_websocket_url(&self, owner_name: &str, project_name: &str, exp_num: i32) -> String {
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

    pub fn create_experiment_run_websocket(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
    ) -> Result<WebSocketClient, WebSocketError> {
        let mut ws_client = WebSocketClient::new();

        let ws_endpoint = self.format_websocket_url(owner_name, project_name, exp_num);

        ws_client
            .connect(
                ws_endpoint,
                &self.session_cookie.clone().unwrap_or("".to_string()),
            )
            .map_err(|e| WebSocketError::ConnectionError(e.to_string()))?;

        Ok(ws_client)
    }

    /// Cancel an experiment.
    ///
    /// The client must be logged in before calling this method.
    pub fn cancel_experiment(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
    ) -> Result<(), ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/cancel"
        ));

        self.post(url, None::<()>)
    }
}
