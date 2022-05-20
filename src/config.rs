use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    api_key: String,
    pub user_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            user_id: "".to_string(),
        }
    }
}
