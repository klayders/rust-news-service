use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ExampleQueryRequest {
    pub title: String,
}
