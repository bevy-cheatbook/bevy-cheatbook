# VSCode

If you are a VSCode user / have suggestions for how to improve this page,
file an issue on [GitHub](https://github.com/bevy-cheatbook/bevy-cheatbook)
or ping me on Discord (`@Ida Iyes#0981`).

---

## Debugger

When running a Bevy app in some way other than `cargo run` (such as inside
a debugger), you need to set the `CARGO_MANIFEST_DIR` environment variable,
as Bevy relies on it to find and load your assets.

Add this to your VSCode run configuration:

```json
    "env": {
        "CARGO_MANIFEST_DIR": "${workspaceFolder}",
    }
```

Here is a complete snippet showing how to create a run configuration for
debugging Bevy (with lldb):

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
