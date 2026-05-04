pub mod request;
pub mod response;
pub mod websocket;

pub use request::{
    CreateExperimentRequest, ListExperimentsQuery, MetricAggregatedQuery, MetricSummaryQuery,
};
pub use response::{
    ExperimentInputResponse, ExperimentListResponse, ExperimentResponse, MetricEntryResponse,
    MetricGroupResponse, MetricMetadataResponse, MetricResponse, MetricSummaryGroupResponse,
    MetricSummaryResponse, TrainingFunctionResponse,
};

use crate::{ClientError, WebSocketClient, transport::ApiTransport, websocket::WebSocketError};

pub struct ExperimentClient<'a> {
    transport: &'a ApiTransport,
}

impl<'a> ExperimentClient<'a> {
    pub(crate) fn new(transport: &'a ApiTransport) -> Self {
        Self { transport }
    }

    pub fn create(
        &self,
        request: CreateExperimentRequest,
    ) -> Result<ExperimentResponse, ClientError> {
        self.transport.post_json("experiments", Some(request))
    }

    pub fn get(&self, experiment_num: i32) -> Result<ExperimentResponse, ClientError> {
        self.transport
            .get_json(format!("experiments/{experiment_num}"))
    }

    pub fn list(&self, query: ListExperimentsQuery) -> Result<ExperimentListResponse, ClientError> {
        let mut url = self.transport.join("experiments");
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(page) = query.page {
                pairs.append_pair("page", &page.to_string());
            }
            if let Some(per_page) = query.per_page {
                pairs.append_pair("per_page", &per_page.to_string());
            }
        }

        self.transport.get_json(url)
    }

    pub fn latest(&self) -> Result<Option<ExperimentResponse>, ClientError> {
        self.transport.get_json("experiments/latest")
    }

    pub fn metric_metadata(
        &self,
        experiment_num: i32,
    ) -> Result<MetricMetadataResponse, ClientError> {
        self.transport
            .get_json(format!("experiments/{experiment_num}/metrics/metadata"))
    }

    pub fn metric_summary(
        &self,
        experiment_num: i32,
        query: MetricSummaryQuery,
    ) -> Result<Option<MetricSummaryResponse>, ClientError> {
        let mut url = self
            .transport
            .join(&format!("experiments/{experiment_num}/metrics/summary"));
        url.query_pairs_mut().append_pair("metric", &query.metric);

        self.transport.get_optional_json(url)
    }

    pub fn metrics(
        &self,
        experiment_num: i32,
        query: MetricAggregatedQuery,
    ) -> Result<Option<MetricResponse>, ClientError> {
        let mut url = self
            .transport
            .join(&format!("experiments/{experiment_num}/metrics"));
        url.query_pairs_mut()
            .append_pair("metric", &query.metric)
            .append_pair("max_points", &query.max_points.to_string())
            .append_pair(
                "downsampling_factor",
                &query.downsampling_factor.to_string(),
            );

        self.transport.get_optional_json(url)
    }

    pub fn websocket_url(&self, experiment_num: i32) -> String {
        let mut url = self
            .transport
            .join(&format!("experiments/{experiment_num}/ws"));
        url.set_scheme(if self.transport.base_url().scheme() == "https" {
            "wss"
        } else {
            "ws"
        })
        .expect("Should be able to set ws scheme");

        url.to_string()
    }

    pub fn create_run_websocket(
        &self,
        experiment_num: i32,
    ) -> Result<WebSocketClient, WebSocketError> {
        let mut ws_client = WebSocketClient::new();
        ws_client.connect(&self.websocket_url(experiment_num), self.transport.auth())?;

        Ok(ws_client)
    }
}
