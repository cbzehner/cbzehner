# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

# List available recipes
default:
  @just --list

# Build the web target
build:
    wasm-pack build --target web

# Serve the static index.html
serve:
    python3 -m http.server 8000

# Watch the source code for changes and automatically rebuild. Manual browser refresh required.
watch:
    cargo watch -s "just build && just serve"