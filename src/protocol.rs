use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct DevtoolPage {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    pub devtools_frontend_url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: String,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "result")]
    pub result: EvaluateResult,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateResult {
    #[serde(rename = "result")]
    pub result: Value,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<Value>,
}
