extern crate rusty_v8;
use rusty_v8 as v8;
use std::io::{stdin, stdout, Write};

pub fn input(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let mut res = String::new();
    let ques = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    print!("{}", ques);
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
    rv.set(v8::String::new(scope, &res).unwrap().into());
}
