# Direct World Access

{{#include ../include/links.md}}

The [`World`][bevy::World] is where Bevy ECS stores all data and
associated metadata. It keeps track of [resources][cb::res], [entities and
components][cb::ec].

Typically, the [`App`][bevy::App]'s schedule runner will run all
[stages][cb::stage] (which, in turn, run their [systems][cb::system])
on the main world. Regular [systems][cb::system] are limited in
what data they can access from the world, by their [system parameter
types][builtins::systemparam]. Operations that manipulate the world itself
are only done indirectly using [`Commands`][cb::commands]. This is how most
typical Bevy user code behaves.

However, there are also ways you can get full direct access to the world,
which gives you full control and freedom to do anything with any data stored
in the Bevy ECS:
 - [Exclusive systems][cb::exclusive]
 - [`FromWorld`][bevy::FromWorld] impls
 - Via the [`App`][bevy::App] [builder][cb::app]
 - Manually created [`World`][bevy::World]s for purposes like [tests][cb::system-tests] or scenes
 - Custom Commands
 - Custom [`Stage`][bevy::Stage] impls (not recommended, prefer exclusive systems)

Direct world access lets you do things like:
 - Freely spawn/despawn entities, insert/remove resources, etc., taking effect immediately
   (no delay like when using [`Commands`] from a regular [system][cb::system])
 - Access any component, entities, and resources you want
 - Manually run systems or stages

## Working with the `World`

Here are some ways that you can make use of the direct world access APIs.

### `SystemState`

The easiest way to do things is using a [`SystemState`][bevy::SystemState].

This is a type that "imitates a system", behaving the same way as a
[system][cb::system] with various parameters would. All the same behaviors
like [queries][cb::query], [change detection][cb::change-detection], and
even [`Commands`][cb::commands] are available. You can use any [system
params][builtins::systemparam].

It also tracks any persistent state, used for things like [change
detection][cb::change-detection] or caching to improve performance. Therefore,
if you plan on reusing the same [`SystemState`][bevy::SystemState] multiple
times, you should store it somewhere, rather than creating a new one every
time. Every time you call `.get(world)`, it behaves like another "run"
of a system.

If you are using [`Commands`][bevy::Commands], you can choose when you
want to apply them to the world. You need to manually call `.apply(world)`
on the [`SystemState`][bevy::SystemState], to apply them.

```rust,no_run,noplayground
// TODO: write code example
```

### Running a Stage

If you want to run some systems (a common use-case is
[testing][cb::system-tests]), the easiest way is to construct an impromptu
[`SystemStage`][bevy::SystemStage] ([stages][cb::stage]). This way you reuse
all the scheduling logic that Bevy normally does when running systems.

```rust,no_run,noplayground
// TODO: write code example
```

### Navigating by Metadata

The world contains a lot of metadata that allows navigating all the data
efficiently, such as information about all the stored components, entities,
archeypes.

```rust,no_run,noplayground
// TODO: write code example
```
