use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    pub address: String,
    pub network: String,
}