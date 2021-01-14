use std::io::{stdin, stdout, Write};
use std::process;

pub fn main() {
    // jstime::init(
    //     opt.v8_options
    //         .map(|o| o.split(' ').map(|s| s.to_owned()).collect()),
    // );
    println!("Welcome to Byte v0.0.2");
    println!("Type .help for more information");
    println!("Type .exit to close the REPL");
    loop {
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

        if res == ".exit" {
            process::exit(1);
        } else if res == ".help" {
            replhelp();
        } else if res == ".authors" {
            println!("The Byte Authors :\n* Japroz Saini <sainijaproz@gmail.com> - Core");
        }
    }
}

fn replhelp() {
    println!(
        "
    Commands:
    * .help - Get this message
    * .exit - Exit the REPL
    * .authors - Get the name of the authors who worked on Byte
    "
    );
}
