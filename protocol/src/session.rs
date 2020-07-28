use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionDetails {
    pub id: String,
    pub public_url: String,
}
