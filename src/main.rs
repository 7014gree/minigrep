use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for {}", args);
    println!("In file {}", file_path)
}