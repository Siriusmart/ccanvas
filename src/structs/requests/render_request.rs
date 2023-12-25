use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum RenderRequest {
    #[serde(rename = "set char")]
    SetChar { x: u32, y: u32, c: char },
}
