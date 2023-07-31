{{#include ../../include/header011.md}}

# Visual Studio Code

If you are a VSCode user and you'd like something to be added to this page,
please file a [GitHub Issue][project::cb::issues].

## Rust Language Support

For good Rust support, install the Rust Analyzer plugin. Bevy uses a lot of
procedural macros, so be sure to enable proc-macro support in the RA settings
(it is not enabled by default).

## `CARGO_MANIFEST_DIR`

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

(this is for development on Bevy itself, and testing with the `breakout` example)

(adapt to your needs if using for your project)

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
