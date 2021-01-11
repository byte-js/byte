use std::io::{stdin, stdout, Write};

pub fn main() {
    println!("Welcome to Byte verson 0.0.1");
    println!("Type .help for more information");
    while true {
        let mut res = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut res)
            .expect("Did not enter a correct string");
        if let Some('\n') = res.chars().next_back() {
            res.pop();
        }
        if let Some('\r') = res.chars().next_back() {
            res.pop();
        }
    }
}
