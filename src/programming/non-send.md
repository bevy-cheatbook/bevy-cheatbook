{{#include ../include/header013.md}}

# Non-Send Resources

"Non-send" refers to data types that must only be accessed from the "main
thread" of the application. Such data is marked by Rust as `!Send` (lacking
the [`Send`] trait).

Some (often system) libraries have interfaces that cannot be safely used from
other threads. A common example of this are various low-level OS interfaces
for things like windowing, graphics, or audio. If you are doing advanced
things like creating a Bevy plugin for interfacing with such things, you
may encounter the need for this.

Normally, Bevy works by running all your [systems][cb::system] on a
thread-pool, making use of many CPU cores. However, you might need to ensure
that some code always runs on the "main thread", or access data that is not
safe to access in a multithreaded way.

## Non-Send Systems and Data Access

To do this, you can use a [`NonSend<T>`] / [`NonSendMut<T>`] system parameter.
This behaves just like [`Res<T>`] / [`ResMut<T>`], letting you access an ECS
[resource][cb::res] (single global instance of some data), except that the
presence of such a parameter forces the Bevy scheduler to always run the
[system][cb::system] on the main thread. This ensures that data never has to be
sent between threads or accessed from different threads.

One example of such a resource is [`WinitWindows`] in Bevy.  This is the
low-level layer behind the [window entities][cb::window] that you typically use
for window management. It gives you more direct access to OS window management
functionality.

```rust,no_run,noplayground
{{#include ../code013/src/programming/non_send.rs:nonsend}}
```
```rust,no_run,noplayground
{{#include ../code013/src/programming/non_send.rs:nonsend-app}}
```

## Custom Non-Send Resources

Normally, to insert [resources][cb::res], their types must be [`Send`].

Bevy tracks non-Send resources separately, to ensure that they can only be
accessed using [`NonSend<T>`] / [`NonSendMut<T>`].

It is not possible to insert non-send resources using
[`Commands`][cb::commands], only using [direct World access][cb::world]. This
means that you have to initialize them in an [exclusive system][cb::exclusive],
[`FromWorld`] impl, or from the [app builder][cb::app].

```rust,no_run,noplayground
{{#include ../code013/src/programming/non_send.rs:insert-nonsend}}
```
```rust,no_run,noplayground
{{#include ../code013/src/programming/non_send.rs:insert-nonsend-app}}
```

Or, for simple things, if you don't need a full-blown system:

```rust,no_run,noplayground
{{#include ../code013/src/programming/non_send.rs:insert-nonsend-app-world}}
```
