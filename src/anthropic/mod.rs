pub mod get_models;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
    //first_id: String,
    //has_more: bool,
    //last_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub created_at: String,
    pub display_name: String,
    pub r#type: String,
}
