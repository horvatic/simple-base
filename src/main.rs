use std::path::Path;
use std::fs;

use ::simple_base::engine;
use ::simple_base::command::command_path;

fn main() {
    let addr = "0.0.0.0:8080";
    let exist: bool = Path::new(command_path::DIR).is_dir();
    if !exist {
        match fs::create_dir(command_path::DIR) {
            Ok(_) => engine::runner::run(addr),
            Err(_) => panic!("Can not make dir"),
        }
    }
}