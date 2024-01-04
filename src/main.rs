use std::{env, sync::Arc, time::Duration};

use ccanvas::{
    structs::Space,
    term::{enter, exit},
};
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        println!("Bad arguments: expect `ccanvas [label] [command] (args..)`");
        return;
    }

    let runtime = Runtime::new().unwrap();

    runtime.block_on(enter());

    // creates new master space
    let master = Arc::new(runtime.block_on(Space::new("master".to_string())));
    let handle = runtime.spawn(Space::listen(master.clone()));
    runtime
        .block_on(master.spawn(args[0].clone(), args[1].clone(), args[2..].to_vec()))
        .unwrap();
    runtime.block_on(handle).unwrap();

    // get rid of everyting, kills all processes, etc
    runtime.shutdown_timeout(Duration::from_secs(0));

    exit();
}
