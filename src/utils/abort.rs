extern crate rusty_v8;
use rusty_v8 as v8;
use std::process;

pub fn abort(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() > 0 {
        println!(
            "{}",
            args.get(0)
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope)
        );
    }
    process::exit(1)
}
