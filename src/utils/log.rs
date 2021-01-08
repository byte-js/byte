extern crate rusty_v8;
use rusty_v8 as v8;

pub fn log(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    for i in 0..args.length() {
        let arg = args
            .get(i)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        print!("{}", arg);
    }
    println!("");
}
