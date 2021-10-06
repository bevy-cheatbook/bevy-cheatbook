# Text Editor / IDE

This page contains tips for different text editors and IDEs.

Bevy is, for the most part, like any other Rust project. If your editor/IDE
is set up for Rust, that might be all you need. This page contains additional
information that may be useful for Bevy specifically.

If you have any suggestions for how to improve this page, file an issue on
[GitHub](https://github.com/bevy-cheatbook/bevy-cheatbook) or ping me on
Discord (`@Ida Iyes#0981`).
Your help collecting useful information would be much appreciated! <3

## `CARGO_MANIFEST_DIR`

When running your app/game, Bevy will search for the `assets` folder in
the path specified in the `CARGO_MANIFEST_DIR` environment variable. This
allows `cargo run` to work correctly from the terminal.

If you are using your editor/IDE to run your project in a non-standard way
(say, inside a debugger), you have to be sure to set that correctly.

## VSCode

Here is a snippet showing how to create a run configuration for debugging Bevy
(with lldb):

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

