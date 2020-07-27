use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionDetails {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub details: Option<SessionDetails>,
}
