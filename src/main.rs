use std::io::{Write, stdin, stdout};
fn main() {
    println!("Welcome to My database :)!");
    loop {
        print!(">> ");
        let _ = Write::flush(&mut stdout());
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let commands: Vec<&str> = input.split_whitespace().collect();
        if commands.is_empty() {
            continue;
        }
        if commands[0] == "exit" {
            break;
        }
        println!("{:?}", commands);
    }
}
