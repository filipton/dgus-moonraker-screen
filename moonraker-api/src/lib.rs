use serde::{Deserialize, Serialize};

pub mod methods;
pub mod params;
pub mod websocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonrakerApiRequest {
    jsonrpc: String,
    method: methods::MoonrakerMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<params::MoonrakerParam>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct MoonrakerMsg {
    pub method: methods::MoonrakerMethod,
    pub params: Option<params::MoonrakerParam>,
}

impl MoonrakerMsg {
    pub fn new(method: methods::MoonrakerMethod, params: Option<params::MoonrakerParam>) -> Self {
        Self { method, params }
    }

    pub fn to_json(&self) -> String {
        let id = methods::get_method_id(&self.method);
        let request = MoonrakerApiRequest {
            jsonrpc: "2.0".to_string(),
            method: self.method.clone(),
            params: self.params.clone(),
            id: Some(id),
        };

        serde_json::to_string(&request).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let request: MoonrakerApiRequest = serde_json::from_str(json)?;
        let method = request.method;
        let params = request.params;

        Ok(Self { method, params })
    }
}
