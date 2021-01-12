// use jstime;
use std::io::{stdin, stdout, Write};

pub fn main() {
    // jstime::init(
    //     opt.v8_options
    //         .map(|o| o.split(' ').map(|s| s.to_owned()).collect()),
    // );
    println!("Welcome to Byte verson 0.0.1");
    println!("Type .help for more information");
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
        println!("{}", res);
        // match jstime.run_script(&res, "REPL") {
        //     Ok(v) => println!("{}", v),
        //     Err(e) => eprintln!("Uncaught: {}", e),
        // }
    }
}
