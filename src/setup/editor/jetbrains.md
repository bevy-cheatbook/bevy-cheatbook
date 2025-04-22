{{#include ../../include/header016.md}}

# JetBrains (RustRover, IntelliJ, CLion)

If you are a JetBrains user and you'd like something to be added to this page,
please file a [GitHub Issue][project::cb::issues].

## Rust Language Support

When using [queries][cb::query], type information gets lost due to Bevy relying
on procedural macros. You can fix this by enabling [procedural macro
support][intellij-rust::6908] in the IDE.

1. type `Experimental feature` in the dialog of the `Help | Find Action` action
2. enable the features `org.rust.cargo.evaluate.build.scripts` and `org.rust.macros.proc`
