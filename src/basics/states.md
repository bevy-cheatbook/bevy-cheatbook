# States

States allow you to structure the runtime "flow" of your app.

You can have different systems running, depending on the state.

You can add systems that only run when transitioning between states.

States are built using stages.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:app-states}}
```
