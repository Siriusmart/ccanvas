use std::process;

use ccanvas::{
    structs::Space,
    term::{enter, exit},
};

#[tokio::main]
async fn main() {
    let mut master = Space::new("master".to_string());

    enter();
    master.listen().await;
    exit();

    process::exit(0);
}
