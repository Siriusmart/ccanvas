use std::{env, process, sync::Arc};

use ccanvas::{
    structs::Space,
    term::{enter, exit},
    values::SCREEN,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        println!("Bad arguments: expect `ccanvas [label] [command] (args..)`");
        return;
    }

    enter().await;

    // creates new master space
    let master = Arc::new(Space::new("master".to_string()).await);
    master
        .spawn(args[0].clone(), args[1].clone(), args[2..].to_vec())
        .await
        .unwrap();
    Space::listen(master.clone()).await;
    exit();

    drop(unsafe { SCREEN.take() }); // this restores the terminal
    drop(master);
    process::exit(0); // this will kills all running tokio tasks, and immediately exit
}
