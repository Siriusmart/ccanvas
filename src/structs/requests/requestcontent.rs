use serde::Deserialize;

/// variations of requests
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum RequestContent {}

impl RequestContent {
    /// handle the request
    pub fn run(&self) -> Result<(), crate::Error> {
        todo!()
    }
}
