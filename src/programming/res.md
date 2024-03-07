{{#include ../include/header013.md}}

# Resources

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Resources allow you to store a single global instance of some data type,
independently of [entities][cb::ec].

Use them for [data][cb::ecs-intro-data] that is truly global for your app, such
as configuration / settings. Resources make it easy for you to access such data
from anywhere.

---

To create a new resource type, simply define a Rust `struct` or `enum`, and
derive the [`Resource`][bevy::Resource] trait, similar to
[components][cb::component] and [events][cb::event].

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:struct}}
```

Types must be unique; there can only be at most one instance of a given type. If
you might need multiple, consider using [entities and components][cb::ec] instead.

Bevy [uses resources for many things][builtins::res]. You can use these builtin
resources to access various features of the engine. They work just like your own
custom types.

## Accessing Resources

To access the value of a resource from [systems][cb::system], use `Res`/`ResMut`:

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:systemparam}}
```

## Managing Resources

If you need to create/remove resources at runtime, you can do so using
[commands][cb::commands] ([`Commands`][bevy::Commands]):

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:commands}}
```

Alternatively, using [direct World access][cb::world] from an [exclusive
system][cb::exclusive]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:exclusive}}
```

Resources can also be set up from the [app builder][cb::app]. Do this for
resources that are meant to always exist from the start.

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:app}}
```

## Resource Initialization

If you want to be able to use `.init_resource` to create your resource,
here is how you can provide the default value.

Implement [`Default`][std::Default] for simple resources:

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:default}}
```

For resources that need complex initialization, implement [`FromWorld`][bevy::FromWorld]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/res.rs:fromworld}}
```

Beware: it can be easy to get yourself into a mess of unmaintainable code
if you overuse [`FromWorld`][bevy::FromWorld] to do complex things.

## Usage Advice

The choice of when to use [entities/components][cb::ec] vs. resources is
typically about how you want to access the [data][cb::ecs-intro-data]: globally
from anywhere (resources), or using ECS patterns (entities/components).

Even if there is only one of a certain thing in your game (such as the
player in a single-player game), it can be a good fit to use an entity
instead of resources, because entities are composed of multiple components,
some of which can be common with other entities. This can make your game
logic more flexible. For example, you could have a "health/damage system"
that works with both the player and enemies.

### Settings

One common usage of resources is for storing settings and configuration.

However, if it is something that cannot be changed at runtime and only used when
initializing a [plugin][cb::plugin], consider putting that inside the plugin's
`struct`, instead of a resource.

### Caches

Resources are also useful if you want to store some data in a way that is easier
or more efficient for you to access. For example, keeping a collection of [asset
handles][cb::handle], or using a custom datastructure for representing a game
map more efficiently than using entities and components, etc.

[Entities and Components][cb::ec], as flexible as they are, are not necessarily
the best fit for all use cases. If you want to represent your data some other
way, feel free to do so. Simply create a resource and put it there.

### Interfacing with external libraries

If you want to integrate some external non-Bevy software into a Bevy app,
it can be very convenient to create a resource to hold onto its state/data.

For example, if you wanted to use an external physics or audio engine, you
could put all its data in a resource, and write some systems to call its
functions. That can give you an easy way to interface with it from Bevy code.

If the external code is not thread-safe (`!Send` in Rust parlance), which is
common for non-Rust (e.g C++ and OS-level) libraries, you should use a
[Non-Send][cb::nonsend] Bevy resource instead. This will make sure any Bevy
system that touches it will run on the main thread.
