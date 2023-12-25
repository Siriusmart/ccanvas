use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum ResponseError {
    #[serde(rename = "component not found")]
    ComponentNotFound
}
