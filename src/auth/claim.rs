use serde::{Deserialize, Serialize};

use super::google::GoogleId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub id: GoogleId,
}
