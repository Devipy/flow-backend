use chrono::{DateTime, Utc};
use flow_lib::{FlowRunId, NodeId, UserId};
use serde::{Deserialize, Serialize};

pub mod apikey;
pub mod config;
pub mod connection;
pub mod error;
pub mod local_storage;
pub mod pool;
pub mod wasm_storage;

pub use deadpool_postgres::Client as DeadPoolClient;
pub use error::{Error, Result};
pub use local_storage::LocalStorage;
pub use tokio_postgres::error::SqlState;
pub use wasm_storage::{StorageError, WasmStorage};

#[derive(Serialize, Deserialize)]
pub struct FlowRunLogsRow {
    pub user_id: UserId,
    pub flow_run_id: FlowRunId,
    pub log_index: i32,
    pub node_id: Option<NodeId>,
    pub times: Option<i32>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub log_level: String,
    pub content: String,
    pub module: Option<String>,
}
