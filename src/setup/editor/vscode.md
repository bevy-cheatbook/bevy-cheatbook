{{#include ../../include/header016.md}}

# Visual Studio Code

If you are a VSCode user and you'd like something to be added to this page,
please file a [GitHub Issue][project::cb::issues].

## Rust Language Support

For good Rust support, install the Rust Analyzer plugin.

### Speed Up Rust Analyzer

If you have used `.cargo/config.toml` to set a non-default linker for fast
compiles, Rust Analyzer will, unfortunately, ignore it. You need to also
configure RA to use it, with the following setting (in VSCode `settings.json`):

Windows:

```json
"rust-analyzer.cargo.extraEnv": {
    "RUSTFLAGS": "-Clinker=rust-lld.exe"
}
```

Linux (mold):

```json
"rust-analyzer.cargo.extraEnv": {
    "RUSTFLAGS": "-Clinker=clang -Clink-arg=-fuse-ld=mold"
}
```

Linux (lld):

```json
"rust-analyzer.cargo.extraEnv": {
    "RUSTFLAGS": "-Clinker=clang -Clink-arg=-fuse-ld=lld"
}
```

## Run configuration

When running your app/game, Bevy will search for the `assets` folder in the path
specified in the `BEVY_ASSET_ROOT` or `CARGO_MANIFEST_DIR` environment variable.
This allows `cargo run` to work correctly from the terminal.

If you want to run your project from VSCode in a non-standard way (say, inside a
debugger), you have to be sure to set that correctly.

If this is not set, Bevy will search for `assets` alongside the executable
binary, in the same folder where it is located. This makes things easy for
distribution. However, during development, since your executable is located
in the `target` directory where `cargo` placed it, Bevy will be unable to
find the `assets`.

Here is a snippet showing how to create a run configuration for debugging Bevy
(with `lldb`):

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug my game",
    "cargo": {
        "args": [
            "build",
        ],
    },
    "args": [],
    "cwd": "${workspaceFolder}",
    "env": {
        "CARGO_MANIFEST_DIR": "${workspaceFolder}",
    }
}
```

If you are working on Bevy itself, here is a config for running the `breakout` example:

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug example 'breakout'",
    "cargo": {
        "args": [
            "build",
            "--example=breakout",
            "--package=bevy"
        ],
        "filter": {
            "name": "breakout",
            "kind": "example"
        }
    },
    "args": [],
    "cwd": "${workspaceFolder}",
    "env": {
        "CARGO_MANIFEST_DIR": "${workspaceFolder}",
    }
}
```

To support dynamic linking, you should also add the following, inside the `"env"` section:

Linux:

```json
"LD_LIBRARY_PATH": "${workspaceFolder}/target/debug/deps:${env:HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib",
```

(replace `stable-x86_64-unknown-linux-gnu` if you use a different toolchain/architecture)

Windows: I don't know. If you do, please [file an issue][project::cb::issues]!
