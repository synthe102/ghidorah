use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Labels = HashMap<String, String>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub labels: HashMap<String, String>,
    pub plugins: Vec<Plugin>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub name: String,
    pub version: String,
}

impl NodeInfo {
    pub fn new(labels: Labels, plugins: Vec<Plugin>, type_field: String) -> Self {
        NodeInfo {
            labels,
            plugins,
            type_field,
        }
    }
    pub fn default() -> Self {
        NodeInfo {
            labels: Labels::new(),
            plugins: Vec::new(),
            type_field: String::from("basic_info"),
        }
    }
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
