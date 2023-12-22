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
    master
        .spawn("test".to_string(), "ccanvas-component".to_string(), vec![])
        .await
        .unwrap();
    master.listen().await;
    exit();

    drop(unsafe { SCREEN.take() }); // this restores the terminal
    drop(master);
    process::exit(0); // this will kills all running tokio tasks, and immediately exit
}
