use rusty_v8 as v8;
use std::env;
use std::io::Read;
use std::process;
// Cli messages import
mod cli;
use cli::help;
// File system imports
mod fs;
//Repl imports
mod repl;
//Utility function imports
mod utils;
use utils::abort;
// use utils::args;
use utils::exec;
use utils::input;
use utils::log;

pub fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut code = String::new();
    if argv.len() == 1 {
        repl::main();
        return;
    }
    if argv[1].clone() == "run" && argv.len() == 3 {
        let mut file = std::fs::File::open(argv[2].clone()).unwrap();
        file.read_to_string(&mut code).unwrap();
    } else if argv[1].clone() == "run" && argv.len() == 3 {
        println!("Usage : byte run <filename>");
        process::exit(1);
    } else if argv[1].clone() == "repl" {
        repl::main();
        return;
    }

    if argv[1].clone() == "help" || argv[1].clone() == "-h" {
        help::main();
        return;
    } else if argv[1].clone() == "-V" || argv[1].clone() == "--version" {
        println!("Byte v0.0.1");
        return;
    }
    // Create the platform
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Create a new scope for safe execution of untrusted code
    // without any exploits on the system
    {
        //Create the isolate(code executer)
        let mut isolate = v8::Isolate::new(Default::default());
        let handle_scope = &mut v8::HandleScope::new(&mut isolate);
        let context = v8::Context::new(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);
        //Create an object template
        //* : All our functions
        // An object template can call our function from rust
        let object_template = v8::ObjectTemplate::new(scope);

        //Print function
        let log_tmpl = v8::FunctionTemplate::new(scope, log::log);
        let log = v8::String::new(scope, "log").unwrap();
        object_template.set(log.into(), log_tmpl.into());

        //Input function
        let input_tmpl = v8::FunctionTemplate::new(scope, input::input);
        let input = v8::String::new(scope, "input").unwrap();
        object_template.set(input.into(), input_tmpl.into());

        let exec_tmpl = v8::FunctionTemplate::new(scope, exec::exec);
        let exec = v8::String::new(scope, "exec").unwrap();
        object_template.set(exec.into(), exec_tmpl.into());

        let abort_tmpl = v8::FunctionTemplate::new(scope, abort::abort);
        let abort = v8::String::new(scope, "abort").unwrap();
        object_template.set(abort.into(), abort_tmpl.into());

        // let hd_tmpl = v8::FunctionTemplate::new(scope, homedir::main);
        // let hd = v8::String::new(scope, "homedir").unwrap();
        // object_template.set(hd.into(), hd_tmpl.into());

        // let function_template = v8::FunctionTemplate::new(scope, args::args);
        // let name = v8::String::new(scope, "args").unwrap();
        // object_template.set(name.into(), function_template.into());

        //Create a content from our object template
        let context = v8::Context::new_from_template(scope, object_template);
        let scope = &mut v8::ContextScope::new(scope, context);

        let source = v8::String::new(scope, &code).unwrap();
        let script = v8::Script::compile(scope, source, None).unwrap();

        //Run the script
        script.run(scope);
    }
}
