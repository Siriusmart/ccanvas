use std::process;

use ccanvas::{
    structs::Space,
    term::{enter, exit},
    values::SCREEN,
};

#[tokio::main]
async fn main() {
    enter().await;

    // creates new master space
    let mut master = Space::new("master".to_string()).await;
    master.listen().await;
    exit();

    drop(unsafe { SCREEN.take() }); // this restores the terminal
    process::exit(0); // this will kills all running tokio tasks, and immediately exit
}
