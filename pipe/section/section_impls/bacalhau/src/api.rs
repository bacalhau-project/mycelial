use crate::StdError;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct SubmitErrorResponse {
    error: String,
    #[allow(dead_code)]
    message: String,
}

#[derive(Deserialize)]
struct SubmitResponse {
    #[serde(rename = "JobID")]
    job_id: String,

    #[serde(rename = "EvaluationID")]
    _evaluation_id: String,

    #[serde(rename = "Warnings")]
    _warnings: Option<Vec<String>>,
}

pub async fn submit(body: &str) -> Result<String, StdError> {
    // For now we'll assume a local instance and using the newer version
    // of the jobs API.
    let api_url = "http://127.0.0.1:20000/api/v1/orchestrator/jobs";
    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .body(body.to_owned())
        .send()
        .await?;

    let success = &response.status().is_success();
    let resp_body = &response.bytes().await?;

    match success {
        true => {
            let sresp: SubmitResponse = serde_json::from_slice(resp_body.as_ref()).unwrap();
            Ok(sresp.job_id)
        }
        _ => {
            let err: SubmitErrorResponse = serde_json::from_slice(resp_body.as_ref()).unwrap();
            Err(err.error.into())
        }
    }
}
