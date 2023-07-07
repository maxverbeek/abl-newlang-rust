# An implementation of the API example in Rust

This repository implements the synchronous API example in Rust. While I think
it's implemented fairly cleanly, the implementation required a fair amount of
documentation reading. I estimate that it took around 1.5 hours to make this.

## Compiling and running

After installing cargo through rustup, `cargo run` in this directory should run
the program on `0.0.0.0:3000`. A nix flake that installs rust and cargo is
available, which can be used for the same thing:

```sh
echo "use flake" > .envrc
```
