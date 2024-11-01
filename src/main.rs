use std::env;
use std::process::exit;
use minigrep::{run, Config};

/*

We can use the collect function to create many kinds of collections, so we explicitly annotate the type of args to specify that we want a vector of strings. Although you very rarely need to annotate types in Rust, collect is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want.

*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    println!("Searching for '{}' ", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        println!("Application Error : {e}");
        exit(1);
    }
}
