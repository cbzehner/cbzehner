+++
title = "Building stress"
date = 2020-06-28

[taxonomies]
tags = ["rust", "commandline"]
categories = ["programming"]
+++

# A simple command-line tool in Rust

I previously [introduced the `stress` utility](https://www.cbzehner.com/introducing-stress/), a simple way to run a command and detect failures.

Today we're going to build it together!

Despite the name, this should be a relaxing tour through the world of command-line programming. If you're stuck at any point, reach out by [opening an issue](https://github.com/cbzehner/cbzehner/issues) and I'll do my best to help you figure it out.

## What are we building?

We're going to build a binary program in [Rust](https://www.rust-lang.org/) which takes in a command and runs it a set number of times. Afterwards, we'll provide some insight about runs!

We'll build a working version, then do a couple refactors to make it all more maintainable.

# What is an error?

We'll rely on [exit codes](https://en.wikipedia.org/w/index.php?title=Exit_code), numbers returned by commands when they finish running, to determine whether a command failed. By convention `0` indicates success and any non-zero values between `1` and `255` indicate failure. [Operating](https://www.gnu.org/software/libc/manual/html_node/Error-Codes.html#index-ENOBUFS-117) [systems](https://www.freebsd.org/cgi/man.cgi?query=errno&apropos=0&sektion=0&manpath=FreeBSD+12.1-RELEASE&arch=default&format=html) provide definitions for the failure codes, but rely on convention to enforce usage.

For our purposes, we'll assume that all commands are using exit codes correctly.<sup>[1](#correct-exit-codes)</sup> That means we can simply run the command and check its exit code to see whether it succeeded or failed!

Relying on a standard like exit codes allows our program to work with commands in any programming language as long as that command follows the exit code convention.<sup>[2](#incorrect-usage)</sup>

# Setup

## Install Rust

If you haven't already, [install Rust](https://www.rust-lang.org/tools/install) and confirm you have a working installation by running `rustc --version`.

## Create our project

Rust comes with a [package manager `cargo`](https://doc.rust-lang.org/cargo/). We're going to use cargo to create a new project with the binary (`--bin`) flag because this program is a executable, where users directly run the final binary, rather than a library, which other programmers embed into their applications.

Navigate to the directory where you want to keep this program, then run `cargo new stress-tutorial --bin`. This will create a new folder `stress-tutorial/` which we can navigate to with `cd stress-tutorial`.

Now that we have access to our project, we can use three different cargo commands:

1. `cargo check` will run Rust typechecking, running the compiler without generating the final code. Give it a try!

   You should see `Finished dev [unoptimized + debuginfo] target(s) in 0.00s` not very exciting...

2. `cargo build` will do everything `cargo check` does as well as saving the code to the `target/` folder.

   Run `cargo build`, then `./target/debug/stress-tutorial` to run your program and you'll see `Hello, world!`!

3. `cargo run` combines the build step above with the manual step of running the output -- `cargo` runs both `build` and j`./target/debug/stress-tutorial` for us!

   Try it out! You'll see `Hello, world!` once again!

# Reading exit codes in a loop

We'll start out by running a hardcoded command and printing a message based on its exit code!

In our `main.rs` file we're going to add `use std::process::Command` ([docs](https://doc.rust-lang.org/std/process/struct.Command.html)) to the top of the file so we can use this library in our code. We'll use `Command` to run a shell command `sh -c exit 0` which immediately quits the command with an exit code of `0`.

Let's create a new command and set it to a variable named `output`. In order to execute the command, we need to call [`output()`](https://doc.rust-lang.org/std/process/struct.Command.html#method.output), but this only gives us a `Result<Output>`. We get a `Result` back because the command we call might fail. For now, let's just chain an `unwrap()` ([docs](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap)) call onto our `Command`. This will panic if the value isn't `Some(<value>)`, crashing our program. 😬

```rust
let output = Command::new("sh")
            .arg("-c")
            .arg(format!("exit {}", 0))
            .output()
            .unwrap();
```

Now we can inspect the `output` of our command `sh -c exit 0` to see whether it was successful or not.

```rust
match output.status.code() {
    Some(0) => println!("Success"),
    _ => println!("Failure"),
}
```

This will print out "Success" if our command had an exit code code of `0` and "Failure" in any other case.

Try `cargo run`, you should see a single "Success" message in the console.

However, this isn't enough, our original goal was to automate running the command multiple times. Let's make the computer do this for us!

Let's wrap a loop around the code we just wrote.

```rust
for _ in 0..10 {
    ... // Your code goes here.
}
```

Now your `main.rs` file should look like

```rust
use std::process::Command;

fn main() {
    for _ in 0..10 {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("exit {}", 0))
            .output()
            .unwrap();
        match output.status.code() {
            Some(0) => println!("Success"),
            _ => println!("Failure"),
        }
    }
}
```

Try it out! What happens if you change the exit code from `0` to another number?

# Adding configuration

Hardcoding various exit codes isn't very exciting though. Let's live dangerously and let our users decide what command to run!

That means we need to read command-line arguments. Luckily, the Rust ecosystem already has a great command-line argument parser (C.L.A.P.) named...you guessed it, `clap`!

However, using `clap` ([link](https://clap.rs/)) requires building the a `clap::App` struct, which can get pretty verbose. We're instead going to take advantage of another crate `structopt` ([link](https://github.com/TeXitoi/structopt)), which derives the command-line arguments and parses them based on Rust structs that we define.

# User-centric Design

It's time to take a step back and think about how we want users to interact with our program. Good rules of thumb are

1. It should be easy to accomplish the goal
1. Obvious things should be possible
1. As little as possible should be necessary

In this case, the goal of the program is to run a command a lot of times and see if it fails.

...but what is a lot? Once? Definitely not! What about 10 times? 1337 times? It's not clear.

This is a good sign that "a lot" should be something we allow users to configure, per rule #2. But we should not require it to be configured (rule #3).

Let's set a default value of 10. This is small enough that if a users program runs slooowly it shouldn't cause much inconvenience. If we set a value of 1,000, a slow-running program might be quite annoying. We expect most users will tweak this setting to match their specific needs, but we can try to be helpful before we meet them!

And of course we want to actually pass in a command to run!

Something like `stress-tutorial --runs 300 yarn jest --test path/to/my/test`

- `stress-tutorial` is the name of our program, we'll be calling this `cargo run` for the most part during development
- `--runs 300` allows the user to specify the number of times to run the program. This should be strictly optional
- `yarn jest --test path/to/my/test` is the actual command the user wants to run.

#### Of commands and lines

Due to the fact that our program is taking in an arbitrary command we're going to have problems with the program thinking that `--test` (from our example above) is being passed to our program, rather than an argument passed to `yarn jest`. To avoid this we can place a `--` before our main command.

> There is a special command line argument which is two hyphens --, and when this is used, special command line handling is disabled from that point onwards, which means all subsequent arguments are considered part of a task description:
>
> \- [Escaping Command Line Characters
> ](https://taskwarrior.org/docs/escapes.html)

# Providing Structure

In order to do this we need to add the `structopt` crate to our `Cargo.toml` file.

```diff
[package]
name = "stress-tutorial"
version = "0.1.0"
authors = ["Chris Zehner <cbzehner@commure.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
+structopt = "0.3.15"
```

Now we can import the crate into our `main.rs`

```diff
use std::process::Command;
+ use structopt::StructOpt;
```

Awesome! We're ready to add a new struct above our `main` method

```rust
/// Put your programs to the test. Run a command in a loop and collect failures.
#[derive(StructOpt)]
struct Cli {
    #[structopt(required = true, min_values = 1, verbatim_doc_comment)]
    /// The command to run. Precede this command with -- in order to pass in flags.
    /// Usage:
    ///   stress --runs 10 -- echo "hello world"
    ///   stress -- ls -a
    cmd: Vec<String>,
    /// The number of times to run the command
    #[structopt(short, long, default_value = "10")]
    runs: i8,
}
```

Let's break this down line-by-line.

```rust
/// Put your programs to the test. Run a command in a loop and collect failures.
```

This is a doc-comment. Rust will include these in the automatically generated documentation. `structopt` additionally includes this in our command-line utility when passed `--help`!

```rust
#[derive(StructOpt)]
```

Tell Rust that it should automatically generate code for the `Cli` struct using the `structopt` crate.

```rust
struct Cli {
    // --snip--
}
```

Define a new Rust struct, `Cli`, short for command-line interface, which is what we're deriving after all!

```rust
    #[structopt(required = true, min_values = 1, verbatim_doc_comment)]
    /// The command to run. Precede this command with -- in order to pass in flags.
    /// Usage:
    ///   stress --runs 10 -- echo "hello world"
    ///   stress -- ls -a
    cmd: Vec<String>,
```

Tell `#[structopt]` that this command is `required`, must have at least one value in the `Vec<String>` and to not alter the format of the doc-comment when parsing it.

The argument passed into our program will be a series of strings seperated by spaces. We could also have structured this as `cmd: String` but because the `std::process::Command` takes a command to execute and it's arguments as seperate pieces using the `Vect<String>` structure allows us to rely on `structopt` to split the components out based on space-separators.

```rust
    /// The number of times to run the command
    #[structopt(short, long, default_value = "10")]
    runs: i8,
```

Finally, allow users to specify the number of runs they want to observe. We specific `#[structopt(..)]` again to have it generate some additional things.

- `short` adds a `-r` argument to our program based on the name `runs`
- `long` adds a `--runs` argument to our program, again based on the name of the field
- `default_value` does what it implies and allows us to omit the `--runs` flag

# Help!

If you try running `cargo run -- --help`, nothing happens? That's because we haven't actually used our `Cli` anywhere!

Add this to the very first line of the `main` method in `main.rs`:

```diff
fn main() {
+    let args = Cli::from_args();
    // --snip--
}
```

and rerun the command above to see

```bash
stress-tutorial 0.1.0
Put your programs to the test. Run a command in a loop and collect failures

USAGE:
    stress-tutorial [OPTIONS] <cmd>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --runs <runs>    The number of times to run the command [default: 10]

ARGS:
    <cmd>...    The command to run. Precede this command with -- in order to pass in flags.
                Usage:
                  stress --runs 10 -- echo "hello world"
                  stress -- ls -a
```

It looks good, but if you try passing in any of the arguments, they are ignore in favor of the hardcoded `exit 0` and `10` runs we specified earlier.

# Putting it all together

## Altering the number of runs

Start by changing our program to respect the `--runs` parameter. It's easier than you expect!

```diff
fn main() {
    let args = Cli::from_args();
+    let runs = args.runs;

+    for _ in 0..10 {
-    for _ in 0..runs {
        // --snip--
    }
}
```

Awesome! Let's run `cargo run -- --runs 1` to see it in action!

Wait, what!?!

```bash
error: The following required arguments were not provided:
    <cmd>...

USAGE:
    stress-tutorial <cmd>... --runs <runs>

For more information try --help
```

Remember that we made the `<cmd>` our program will run required? We haven't implemented it yet. Let's comment it out for a second here while we test our `--runs` flag.

```diff
struct Cli {
+    // #[structopt(required = true, min_values = 1, verbatim_doc_comment)]
-    #[structopt(required = true, min_values = 1, verbatim_doc_comment)]
    // --snip--
}
```

And it's working! When I run `cargo run -- --runs 1` I see only a single `Success` printed to my console.

Quickly now, undo our commented out line from `Cli` above. We're going to need that in just a second.

## Accepting a command

To parse out the command passed into our program, we need to split it into two parts, the base command to run and the arguments we want to pass into the command. To do this we'll add the following lines directly below `let runs = ...`:

```rust
let command: &str = &args.cmd[0];
let arguments: Vec<String> = Vec::from(&args.cmd[1..args.cmd.len()]);
```

This takes the first element of our `cmd` and sets it as our base command. Remember we used `#[structopt(...)]` to require at least one value here? That's how we know accessing the zeroth element will be safe!

Then we take everything else from `cmd` and use it as the arguments.

Update `output` to use these newly created variables:

```rust
let output = Command::new(command).args(&arguments).output().unwrap();
```

And finally let's change our `match` statement to either:

1. Continue printing out "Success"
1. Include the error code for "Failure"
1. Warn when there's not an available exit code

```rust
match output.status.code() {
    Some(0) => println!("[Success] Exit 0"),
    Some(err) => println!("[Failure] Exit {}", err),
    _ => println!("[Unknown] Unable to read exit code"),
}
```

That's it! This will allow us to run a program multiple times and see when it fails! We have a working program close to the [initial commit for `stress` itself](https://github.com/cbzehner/stress/commit/aee382fd4355b8f057d212adf8c12d3fd45498d4).

Try it out!

```bash
cargo run -- --runs 1 -- ls -a
[Success] Exit 0
```

and

```
cargo run -- --runs 1 -- ls --gibberish
[Failure] Exit 1
```

# What comes next?

...

# Areas of further improvement

- Better internal error handling
- Hide the print statements when we detect another program, rather than a human, is running `stress`
- Use any of [the](https://lib.rs/crates/sysexit) [exit](https://lib.rs/crates/exitcode) [code](https://lib.rs/crates/exit-code) libraries to provide additional information about the type of failure

### Footnotes

<a name="correct-exit-codes">1</a>: It takes [some careful thought](https://www.shellscript.sh/exitcodes.html) to chain programs together in the terminal so that failures in intermidate steps aren't overwritten by later commands.

<a name="incorrect-usage">2</a>: If a program does not conform to the convention of non-zero exit codes to indicate success, its probably an oversight/bug and worth raising with the program author.
