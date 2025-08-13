use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    pub timeout_secs: Option<u16>,
    pub auth_ignore_pathes: Option<Vec<String>>,
    pub enable_api_doc: Option<bool>,
}

pub trait Server {}
