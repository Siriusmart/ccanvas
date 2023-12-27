use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseError {
    #[serde(rename = "component not found")]
    ComponentNotFound,
    #[serde(rename = "spawn failed")]
    SpawnFailed,
}
