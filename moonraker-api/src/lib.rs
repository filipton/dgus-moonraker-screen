use serde::{Deserialize, Serialize};

pub mod methods;
pub mod params;
pub mod websocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoonrakerMsg {
    MsgResult {
        jsonrpc: String,
        result: serde_json::Value,
        id: u16,
    },

    MsgMethodParam {
        jsonrpc: String,
        method: methods::MoonrakerMethod,
        params: params::MoonrakerParam,
    },

    MsgMethodParamVec {
        jsonrpc: String,
        method: methods::MoonrakerMethod,
        params: Vec<params::MoonrakerParam>,
    },

    MsgMethodParamID {
        jsonrpc: String,
        method: methods::MoonrakerMethod,
        params: params::MoonrakerParam,
        id: u16,
    },

    MsgMethodParamIDVec {
        jsonrpc: String,
        method: methods::MoonrakerMethod,
        params: Vec<params::MoonrakerParam>,
        id: u16,
    },
}

impl MoonrakerMsg {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn new_param_id(method: methods::MoonrakerMethod, params: params::MoonrakerParam) -> Self {
        let id = methods::get_method_id(&method);

        MoonrakerMsg::MsgMethodParamID {
            jsonrpc: "2.0".to_string(),
            method,
            params,
            id,
        }
    }
}
