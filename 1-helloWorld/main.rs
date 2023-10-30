use std::env;

fn main() {
    println!("Hello, world!");

    println!("Given args are following: ");
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        // first arg is the name of the program and path if used in call
        println!("{}", arg);
    }
}
