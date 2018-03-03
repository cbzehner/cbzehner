# Rust: Use stable, install nightly

If you've installed a couple packages using Rust's package manager, [cargo]() you might have run into the following error.

```
#![feature] may not be used on the stable release channel
```

Ugh! So annoying. This means the package you're trying to install relies on features that aren't in stable yet. Mostly likely you'll need to switch your toolchain to nightly (Rust has stable, beta, and nightly). This is actually really easy in Rust!

```
rustup default nightly
cargo install <program>
rustup default stable
```

Or even better, use the [toolchain override shorthand](https://github.com/rust-lang-nursery/rustup.rs#toolchain-override-shorthand) and just run a single command.

```
cargo +nightly install <program>
```

> The rustup toolchain proxies can be instructed directly to use a specific toolchain, a convience for developers who often test different toolchains. If the first argument to cargo, rustc or other tools in the toolchain begins with +, it will be interpreted as a rustup toolchain name, and that toolchain will be preferred...

I just started learning Rust so I don't know if there's a good reason to use `stable` over `nightly` day to day, but thought this was a handy tip.

This is just another example of the care that has gone into designing both the Rust language and the accompanying ecosystem.
