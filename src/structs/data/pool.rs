use std::collections::HashMap;

/// pool of key-value pairs for shared or private access
#[derive(Default)]
pub struct Pool {
    map: HashMap<String, String>,
}
