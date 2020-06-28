+++
title = "Introducing stress"
date = 2020-06-27

[taxonomies]
tags = ["rust", "commandline"]
categories = ["programming"]
+++

# A simple tool for debugging flakes

Introducing [`stress`, a simple tool for debugging inconsistent errors](https://lib.rs/crates/stress). No one likes seeing a test fail in (CI) and then pressing `Up` + `Enter` every 3 minutes locally to catch a flake!<sup>[1](#lovemanuallytesting)</sup>

Try it out! `cargo install stress && stress --output --bail -- ls -a`

## Motivation

This is frustratingly common in tests due to a variety of factors including
- Code-skew as the code changes without updating the tests
- Poor test isolation
- Transient errors as tests interact with multiple pieces of infrastructure. Ex: frontend + backend + database

Often the fix is either ~~deleting the test~~<sup>[2](#deletingflakes)</sup> editing the test code or changing something about the environment where the test is running, increasing available memory or tweaking the config.

## How does it work?

Pass in a command say `ls -a` and it will be run a specified number of times (default: 10). The exit codes encountered and the number of occurrences will be printed out along with the command output (with the `--output` flag). 

If all that's needed is to see if any runs fail there's a `--bail` flag that stops the program on the first command run that ends with a failure (non-zero exit code).

## Why not \<insert-tool-here\>?

Definitely! That's also a great option if it works for you! A while-loop in bash should be sufficient, as one of my coworkers wisely pointed out.

So why do this, you might wonder? Just to scratch my own itch.

Next time I need to debug a test, I can focus on the problem instead of the tooling.

## Thanks for reading

Leave your thoughts and feedback [on GitHub](https://github.com/cbzehner/stress/issues).

If you're using `stress` [let me know](https://github.com/cbzehner/stress/issues) how I can improve it for *you*.

### Footnotes

<a name="lovemanuallytesting">1</a>: If you love manually trying to catch flakes...uh...🤯

<a name="deletingflakes">2</a>: This is obviously a joke! But it's not always the wrong approach. Test flakes, especially in CI, are a burden on your entire engineering team. They slow down continuous development and reduce trust in the test suite. Empowering engineers to turn off flakes forces engineering teams to confront a [tragedy of the commons](https://en.wikipedia.org/wiki/Tragedy_of_the_commons), failed CI runs.
