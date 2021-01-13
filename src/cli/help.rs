use colored::*;

pub fn main() {
    println!(
        "
    Byte v0.0.1
    A JavaScript Runtime Engine that aims for productivity

    Docs: {}
    Bugs: {}

    To start the REPL:
    byte run repl

    To execute a script:
    byte run <path to file>

    USAGE:
        byte [OPTIONS] [SUBCOMMAND]

    {}:
        {}, {}
                Prints help information

        {}, {}
                Prints version information


    {}:
        {}           Read Eval Print Loop
        {}            Run a program given a filename.
    ",
        "https://github.com/byte-js/byte".yellow(),
        "https://github.com/byte-js/byte/issues".yellow(),
        "OPTIONS".yellow(),
        "-h".green(),
        "--help".green(),
        "-V".green(),
        "--version".green(),
        "SUBCOMMANDS".yellow(),
        "repl".cyan(),
        "run".cyan()
    );
}
