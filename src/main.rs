use std::process;

use ccanvas::{
    structs::Space,
    term::{enter, exit}, values::SCREEN,
};

#[tokio::main]
async fn main() {
    // creates new master space
    let mut master = Space::new("master".to_string());

    enter();
    master.listen().await;
    exit();

    drop(unsafe { SCREEN.take() }); // this restores the terminal
    process::exit(0);
}
