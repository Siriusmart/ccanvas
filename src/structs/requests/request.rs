use serde::Deserialize;

use super::RequestContent;

#[derive(Deserialize)]
pub struct Request {
    source: String,
    content: RequestContent,
}
