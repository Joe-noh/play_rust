mod tree;

use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";

    if args.len() >= 2 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree::list(&target, 0);
}
