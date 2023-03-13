Warning: this page has not been updated for Bevy 0.10 yet!

# Text Editor / IDE

{{#include ../include/links.md}}

This page contains tips for different text editors and IDEs.

Bevy is, for the most part, like any other Rust project. If your editor/IDE
is set up for Rust, that might be all you need. This page contains additional
information that may be useful for Bevy specifically.

Please help improve this page by providing suggestions for things to add.

## `CARGO_MANIFEST_DIR`

When running your app/game, Bevy will search for the `assets` folder in
the path specified in the `CARGO_MANIFEST_DIR` environment variable. This
allows `cargo run` to work correctly from the terminal.

If you are using your editor/IDE to run your project in a non-standard way
(say, inside a debugger), you have to be sure to set that correctly.

If this is not set, Bevy will search for `assets` alongside the executable
binary, in the same folder where it is located. This makes things easy for
distribution. However, during development, since your executable is located
in the `target` directory where `cargo` placed it, Bevy will be unable to
find the `assets`.

## VSCode

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

## IntelliJ

When using [queries][cb::query], type information gets lost due to Bevy relying
on procedural macros. You can fix this by enabling [procedural macro
support][intellij-rust::6908] in the IDE.

1. type `Experimental feature` in the dialog of the `Help | Find Action` action
2. enable the features `org.rust.cargo.evaluate.build.scripts` and `org.rust.macros.proc`
