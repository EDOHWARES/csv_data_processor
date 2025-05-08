use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: String,
    pub city: String,
}