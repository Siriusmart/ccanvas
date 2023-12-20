use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum RequestContent {}

impl RequestContent {
    pub fn run(&self) -> Result<(), crate::Error> {
        todo!()
    }
}
