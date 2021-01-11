extern crate rusty_v8;
use rusty_v8 as v8;
use std::process::Command;

pub fn exec(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let command = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    //Execute the command
    let mut cmd = Command::new(command);
    if args.length() > 1 {
        cmd.arg(
            args.get(0)
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope),
        );
    }
    match cmd.output() {
        Ok(o) => {
            //Wrap it in a unsafe block because the
            // vector of bytes can be corrupted
            unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("Error executing the shell command");
            println!("{}", e);
        }
    }
}
