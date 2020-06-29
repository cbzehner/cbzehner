+++
title = "Building stress"
date = 2020-06-28
draft = true

[taxonomies]
tags = ["rust", "commandline"]
categories = ["programming"]
+++

# A simple command-line tool in Rust

I previously [introduced the `stress` utility](https://www.cbzehner.com/introducing-stress/), a simple way to run a command and detect failures.

Today we're going to build it together!

Despite the name, this should be a relaxing tour through the world of command-line programming. If you're stuck at any point, reach out by [opening an issue](https://github.com/cbzehner/cbzehner/issues) and I'll do my best to help you figure it out.

# What are we building?

We're going to build a binary program in [Rust](https://www.rust-lang.org/) which takes in a command and runs it a set number of times. Afterwards, we'll provide some insight about runs!

We'll build a working version, then do a couple refactors to make it all more maintainable.

# What is an error?

We'll rely on [exit codes](https://en.wikipedia.org/w/index.php?title=Exit_code), numbers returned by commands when they finish running, to determine whether a command failed. By convention `0` indicates success and any non-zero values between `1` and `255` indicate failure. [Operating](https://www.gnu.org/software/libc/manual/html_node/Error-Codes.html#index-ENOBUFS-117) [systems](https://www.freebsd.org/cgi/man.cgi?query=errno&apropos=0&sektion=0&manpath=FreeBSD+12.1-RELEASE&arch=default&format=html) provide definitions for the failure codes, but rely on convention to enforce usage.

For our purposes, we'll assume that all commands are using exit codes correctly.<sup>[1](#correct-exit-codes)</sup> That means we can simply run the command and check its exit code to see whether it succeeded or failed!

Relying on a standard like exit codes allows our program to work with commands in any programming language as long as that command follows the exit code convention.<sup>[2](#incorrect-usage)</sup>

# Project Setup

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

## Reading exit codes in a loop

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

## Adding configuration via the command-line



# Areas of further improvement

- Better internal error handling
- Hide the print statements when we detect another program, rather than a human, is running `stress`
- Use any of [the](https://lib.rs/crates/sysexit) [exit](https://lib.rs/crates/exitcode) [code](https://lib.rs/crates/exit-code) libraries to provide additional information about the type of failure

### Footnotes

<a name="correct-exit-codes">1</a>: It takes [some careful thought](https://www.shellscript.sh/exitcodes.html) to chain programs together in the terminal so that failures in intermidate steps aren't overwritten by later commands.

<a name="incorrect-usage">2</a>: If a program does not conform to the convention of non-zero exit codes to indicate success, its probably an oversight/bug and worth raising with the program author.
