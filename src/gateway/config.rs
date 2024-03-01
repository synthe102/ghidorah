use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KongConfig {
    #[serde(rename = "config_hash")]
    pub config_hash: String,
    #[serde(rename = "config_table")]
    pub config_table: ConfigTable,
    pub hashes: Hashes,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigTable {
    #[serde(rename = "_format_version")]
    pub format_version: String,
    #[serde(rename = "_transform")]
    pub transform: bool,
    #[serde(rename = "ca_certificates")]
    pub ca_certificates: Vec<Value>,
    pub certificates: Vec<Value>,
    #[serde(rename = "consumer_group_consumers")]
    pub consumer_group_consumers: Vec<Value>,
    #[serde(rename = "consumer_groups")]
    pub consumer_groups: Vec<Value>,
    pub consumers: Vec<Value>,
    #[serde(rename = "degraphql_routes")]
    pub degraphql_routes: Vec<Value>,
    #[serde(rename = "graphql_ratelimiting_advanced_cost_decoration")]
    pub graphql_ratelimiting_advanced_cost_decoration: Vec<Value>,
    pub licenses: Vec<License>,
    pub parameters: Vec<Parameter>,
    pub plugins: Vec<Value>,
    pub routes: Vec<Value>,
    pub services: Vec<Value>,
    pub snis: Vec<Value>,
    pub targets: Vec<Value>,
    pub upstreams: Vec<Value>,
    pub vaults: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    pub payload: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    #[serde(rename = "_format_version")]
    pub format_version: String,
    #[serde(rename = "_transform")]
    pub transform: String,
    #[serde(rename = "ca_certificates")]
    pub ca_certificates: String,
    pub certificates: String,
    #[serde(rename = "consumer_group_consumers")]
    pub consumer_group_consumers: String,
    #[serde(rename = "consumer_groups")]
    pub consumer_groups: String,
    pub consumers: String,
    #[serde(rename = "degraphql_routes")]
    pub degraphql_routes: String,
    #[serde(rename = "graphql_ratelimiting_advanced_cost_decoration")]
    pub graphql_ratelimiting_advanced_cost_decoration: String,
    pub licenses: String,
    pub parameters: String,
    pub plugins: String,
    pub routes: String,
    pub services: String,
    pub snis: String,
    pub targets: String,
    pub upstreams: String,
    pub vaults: String,
}
