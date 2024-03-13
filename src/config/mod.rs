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
    #[serde(rename = "filter_chains")]
    pub filter_chains: Vec<Value>,
    #[serde(rename = "graphql_ratelimiting_advanced_cost_decoration")]
    pub graphql_ratelimiting_advanced_cost_decoration: Vec<Value>,
    #[serde(rename = "key_sets")]
    pub key_sets: Vec<Value>,
    pub keys: Vec<Value>,
    pub licenses: Vec<License>,
    pub parameters: Vec<Parameter>,
    pub plugins: Vec<Value>,
    pub routes: Vec<Route>,
    pub services: Vec<Service>,
    pub snis: Vec<Value>,
    pub targets: Vec<Target>,
    pub upstreams: Vec<Upstream>,
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
pub struct Route {
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "https_redirect_status_code")]
    pub https_redirect_status_code: i64,
    pub id: String,
    pub name: String,
    #[serde(rename = "path_handling")]
    pub path_handling: String,
    pub paths: Vec<String>,
    #[serde(rename = "preserve_host")]
    pub preserve_host: bool,
    pub protocols: Vec<String>,
    #[serde(rename = "regex_priority")]
    pub regex_priority: i64,
    #[serde(rename = "request_buffering")]
    pub request_buffering: bool,
    #[serde(rename = "response_buffering")]
    pub response_buffering: bool,
    pub service: String,
    #[serde(rename = "strip_path")]
    pub strip_path: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "connect_timeout")]
    pub connect_timeout: i64,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    pub enabled: bool,
    pub host: String,
    pub id: String,
    pub name: String,
    pub path: String,
    pub port: i64,
    pub protocol: String,
    #[serde(rename = "read_timeout")]
    pub read_timeout: i64,
    pub retries: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    #[serde(rename = "write_timeout")]
    pub write_timeout: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    #[serde(rename = "created_at")]
    pub created_at: i64,
    pub id: String,
    pub target: String,
    pub upstream: String,
    pub weight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upstream {
    pub algorithm: String,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "hash_fallback")]
    pub hash_fallback: String,
    #[serde(rename = "hash_on")]
    pub hash_on: String,
    #[serde(rename = "hash_on_cookie_path")]
    pub hash_on_cookie_path: String,
    pub healthchecks: Healthchecks,
    pub id: String,
    pub name: String,
    pub slots: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healthchecks {
    pub active: Active,
    pub passive: Passive,
    pub threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Active {
    pub concurrency: i64,
    pub healthy: Healthy,
    #[serde(rename = "http_path")]
    pub http_path: String,
    #[serde(rename = "https_verify_certificate")]
    pub https_verify_certificate: bool,
    pub timeout: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unhealthy: Unhealthy,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healthy {
    #[serde(rename = "http_statuses")]
    pub http_statuses: Vec<i64>,
    pub interval: i64,
    pub successes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unhealthy {
    #[serde(rename = "http_failures")]
    pub http_failures: i64,
    #[serde(rename = "http_statuses")]
    pub http_statuses: Vec<i64>,
    pub interval: i64,
    #[serde(rename = "tcp_failures")]
    pub tcp_failures: i64,
    pub timeouts: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    pub healthy: Healthy2,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unhealthy: Unhealthy2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healthy2 {
    #[serde(rename = "http_statuses")]
    pub http_statuses: Vec<i64>,
    pub successes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unhealthy2 {
    #[serde(rename = "http_failures")]
    pub http_failures: i64,
    #[serde(rename = "http_statuses")]
    pub http_statuses: Vec<i64>,
    #[serde(rename = "tcp_failures")]
    pub tcp_failures: i64,
    pub timeouts: i64,
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
    #[serde(rename = "filter_chains")]
    pub filter_chains: String,
    #[serde(rename = "graphql_ratelimiting_advanced_cost_decoration")]
    pub graphql_ratelimiting_advanced_cost_decoration: String,
    #[serde(rename = "key_sets")]
    pub key_sets: String,
    pub keys: String,
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

// impl KongConfig {
//     fn new() -> KongConfig {
//         KongConfig {
//             config_hash: String::new(),
//             type_field: String::new(),
//         }
//     }
// }
