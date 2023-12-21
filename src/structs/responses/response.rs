use serde::Deserialize;

use super::ResponseContent;

/// a return signal back to a subprocess
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Response {
    /// the content of the response
    #[serde(flatten)]
    content: ResponseContent,
}
