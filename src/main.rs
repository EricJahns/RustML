mod network;
mod functions;
use network::options;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut options = options::Options::new();
    args.remove(0);
    println!("{:?}", args);
    options.read_input(args);
    println!("{}", options.file_name);
}
