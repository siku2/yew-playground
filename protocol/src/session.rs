use serde::{Deserialize, Serialize};

pub type SessionId = String;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionDetails {
    pub id: SessionId,
    pub public_url: String,
}
