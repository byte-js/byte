// extern crate dirs;
// extern crate rusty_v8;
// use dirs::home_dir;
// use rusty_v8 as v8;
// use std::env::consts;
// use std::process::Command;

// pub fn main(
//     scope: &mut v8::HandleScope,
//     _args: v8::FunctionCallbackArguments,
//     mut rv: v8::ReturnValue,
// ) {
//     // let mut dir_name: String = String::new();
//     // let dir = home_dir();
//     // for (index, i) in dir.clone().iter() {
//     //     dir_name = dir_name + dir[index];
//     // }
//     let mut cmd = Command::new("");
//     if consts::OS == "macos" {
//         cmd = Command::new("ls");
//     }else if consts::OS == "windows"{
//         cmd = Command::new("windows")
//     }

//     let out = cmd.

//     rv.set(v8::String::new(scope, cmd).unwrap().into());
// }
