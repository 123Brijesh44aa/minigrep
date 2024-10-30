use core::panic;
use std::env;
use std::fs;

/*

We can use the collect function to create many kinds of collections, so we explicitly annotate the type of args to specify that we want a vector of strings. Although you very rarely need to annotate types in Rust, collect is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want.

*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}' ", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
 
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config{query,file_path}
    }
}