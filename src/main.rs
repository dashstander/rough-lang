use std::env;
use std::fs;


use rough::scanner::Scanner;


fn run(source: String) {
    let scanner = Scanner { source };
    for c in scanner.source.chars() {
        println!("{}", c)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    let source = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    run(source);
}
