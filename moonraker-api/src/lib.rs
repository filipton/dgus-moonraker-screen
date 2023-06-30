use serde::{Deserialize, Serialize};

pub mod methods;
pub mod params;
pub mod websocket;

pub use methods::{get_method_id, MoonrakerMethod};
pub use params::MoonrakerParam;
pub use websocket::connect;

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
        method: MoonrakerMethod,
        params: MoonrakerParam,
    },

    MsgMethodParamVec {
        jsonrpc: String,
        method: MoonrakerMethod,
        params: Vec<MoonrakerParam>,
    },

    MsgMethodParamID {
        jsonrpc: String,
        method: MoonrakerMethod,
        params: MoonrakerParam,
        id: u16,
    },

    MsgMethodParamIDVec {
        jsonrpc: String,
        method: MoonrakerMethod,
        params: Vec<MoonrakerParam>,
        id: u16,
    },

    MsgMethod {
        jsonrpc: String,
        method: MoonrakerMethod,
    },

    MsgMethodID {
        jsonrpc: String,
        method: MoonrakerMethod,
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

    pub fn new_with_method_and_id(method: methods::MoonrakerMethod) -> Self {
        let id = methods::get_method_id(&method);

        MoonrakerMsg::MsgMethodID {
            jsonrpc: "2.0".to_string(),
            method,
            id,
        }
    }
}
