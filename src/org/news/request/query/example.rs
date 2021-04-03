use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ExampleQueryRequest {
    pub newsTitle: String,
}
