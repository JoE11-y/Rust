use std::env;

// Passing in stuff from command line

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Joe";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
