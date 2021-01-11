# Byte

Byte is a easy and productive runtime for Javascript . It makes making complex programs simple and easy-to-scale with its large and fast `Rust API`

# Aim

The aim for Byte is that modern javascript runtimes are not easy to learn and usually there is a learning curve of about a month to gain knowledge and get comfortable in the software. With Byte, this learning curve is reduced to about a day. With lightning fast `IO` , Byte will attract users rapidly.

# Features

## Speed

Byte was implemented on top of Google's `V8 Javascript Engine` and `Rust` which is a lethal combination . Byte can perform `IO` and can perform `Command Line Tasks` abnormally quickly.

## CLIs

Another reason for making Byte was that making CLI's had become very complex in the past and it had to be made simpler. Byte does just that . With in-built function to take `input`, `abort` with an error code and getting `command-line-arguements` it is very easy to make software for the terminal with Byte.

# Tasks

As you might have guessed, `Byte` still needs a lot of stuff and it is not at all ready for `production` status. That being said, we want more developers to contribute to the development of `Byte` by `creating issues`, making `pull requests` and `reveiwing code`

For all the tasks that are pending, please check the `TODO.md` file and if you have completed one of the tasks, please delete that task fro the file so that other people dont have to implement them again.

# Documentation

# Functions

## `log()`

The `log` function is used to print text onto the terminal screen , just like the `console.log()` function

`log(text : any)`

## Example

```javascript
log("Hello World");
```

```bash
$ cargo run index.js
Hello World
$
```

## `input()`

The `input` function takes an input from the terminal . It required one parameter which is the question or the text that has to be displayed onto the screen.

`input(ques : any)`

```javascript
let res = input("What is your name ? ");
log(`Your name is ${res}`);
```

```bash
$ cargo run index.js
What is your name ? Japroz
Your name is Japroz
$
```

## `exec()`

The `exec` function takes in a command which has to executed in the terminal. The command is not executed in a specific shell, but the shell in which the software is being run. The output of the command is printed to the terminal or an error is printed if the comman could not be executed

`exec(command : string)`

```javascript
exec("ls");
```

```bash
$ cargo run index.js
src/
target/
.cargo/
test/
Cargo.toml
Cargo.lock
$
```

## `args()`

The `args` returns the command line arguements passed to the program
`args()`

```javascript
const argv = args();
log(argv);
```

```bash
$ cargo run index.js hello world
[
  '<path_to_index.js>',
  'hello',
  'world'
]
$
```

## '`abort()`

The process can b stopped with the help of the `abort()` which can take an optional argument `message` which is displayed just before the process is aborted

`abort(message : any)`

```javascript
log("This is the log function");
const res = input("Is HTML a programming language : ");
if (res == "y") {
  abort("You are upto no good!");
}
log(`Correct. Your answer was : ${res}`);
```

```bash
$ cargo run index.js
Is HTML a programming language : y
You are upto no good!
$
```

# Usage

To build the project , make sure that you have the following installed :

```bash
rustup
rustc
cargo
```

## Steps

Firstly, clone the project with the following command

```bash
git clone https://github.com/byte-js/byte.git
```

Then, you will have to change the dircetory of your terminal with the following command

```bash
cd byte
```

Then, to run the project, run

```bash
cargo run test/js/index.js
```

The first time you build the project will take some time because it has to install and build Google's `v8` but after that it will run quickly

After that, run the following command with the path to any `javascript` file that you would like to run:

```bash
cargo run <filename>
```

If you dont have any file, then you can run a file from the `test` folder with the following command:

```bash
cargo run test/js/index.js
```

# Building

## Linux/MacOS/Unix

To build the project, run the following commands inside of the terminal:

```bash
cargo build --release
```

This is going to build it for Linux operating systems. The binary will be located in the `target/release' folder with the name `byte'

## Windows

## `IMPORTANT NOTE`

Currently, `rusty_v8` does not compiler with rust for `Windows`, but we will be rolling out the binaries for `Windows` very soon. You can wait till then, or you can continue with the steps given below and it just might work on your `Windows` machine.

If you are on windows, make sure you have `mingw-w64` installed on your system. After that is installed, run the following commands inside of your terminal inside the project directory:

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

After that, the build will be complete. The binary will be located in the `target/release' folder with the name `byte'

# Developer Help

`Byte` is still in its early stages and we want all the help that we can get. We encourage developers to create `PR's` and join the organisation so that the software can be improved and made better

# Authors

```
name,
email,
github,
twitter
```

```bash
Japroz Singh Saini,
<sainijaproz@gmail.com>,
@Japroz-Saini,
@JaprozSaini
```

# Technical Info

All the info for `Byte` is stored in the `tech.json` file in the root directory of the project.

```json
{
  "name": "Byte",
  "version": "0.0.1",
  "license": "MIT",
  "languages": ["rust", "toml", "MakeFile", "json"],
  "compatibility": ["Windows (64 bit)", "MacOS", "Linux"],
  "authors": [
    {
      "name": "Japroz Singh Saini",
      "email": "sainijaproz@gmail.com",
      "github": "@Japroz-Saini",
      "twitter": "@JaprozSaini"
    }
  ]
}
```
