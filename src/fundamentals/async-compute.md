{{#include ../include/header014.md}}

# Background Computation

Relevant official examples:
[`async_compute`][example::async_compute],
[`external_source_external_thread`][example::external_source_external_thread].

---

Sometimes you need to perform long-running background computations. You want
to do that in a way that does not hold up Bevy's main frame update loop, so
that your game can keep refreshing and feeling responsive with no lag spikes.

To do this, Bevy offers a special [`AsyncComputeTaskPool`]. You can spawn
tasks there, and Bevy will run them on special CPU threads dedicated for
the purpose of running background computations.

When you initiate the task, you get a [`Task`] handle, which you can use
to check for completion.

It is common to write two separate [systems][cb::system], one for initiating
tasks and storing the handles, and one for handling the finished work when
the tasks complete.

```rust,no_run,noplayground
{{#include ../code014/src/fundamentals/async_compute.rs:async-compute}}
```

```rust,no_run,noplayground
{{#include ../code014/src/fundamentals/async_compute.rs:async-compute-app}}
```

### Internal Parallelism

Your tasks can also spawn additional independent tasks themselves, for extra
parallelism, using the same API as shown above, from within the closure.

If you'd like your background computation tasks to process data in parallel,
you can use [scoped tasks][cb::scoped-task]. This allows you to create
tasks that borrow data from the function that spawns them.

Using the scoped API can also be easier, even if you don't need to borrow data,
because you don't have to worry about storing and `await`ing the [`Task`] handles.

A common pattern is to have your main task (the one you initiate from your
[systems][cb::system], as shown earlier) act as a "dispacher", spawning a bunch
of [scoped tasks][cb::scoped-task] to do the actual work.

## I/O-heavy Workloads

If your intention is to do background I/O (such as networking or accessing
files) instead of heavy CPU work, you can use [`IoTaskPool`] instead of
[`AsyncComputeTaskPool`]. The APIs are the same as shown above. The choice
of task pool just helps Bevy schedule and manage your tasks appropriately.

For example, you could spawn tasks to run your game's multiplayer
netcode, save/load game save files, etc. Bevy's [asset loading
infrastructure][chapter::assets] also makes use of the [`IoTaskPool`].

## Passing Data Around

The previous examples showcased a "spawn-join" programming pattern, where
you start tasks to perform some work and then consume the values they return
after they complete.

If you'd like to have some long-running tasks that send values
back to you, instead of returning, you can use channels (from the
[`async-channel`][docs::async-channel] crate). Channels can also be used to
send data to your long-running background tasks.

Set up some channels and put the side you want to access from Bevy in a
[resource][cb::res]. To receive data from Bevy [systems][cb::system],
you should poll the channels using a non-blocking method, like `try_recv`,
to check if data is available.

```rust,no_run,noplayground
{{#include ../code014/src/fundamentals/async_compute.rs:channels}}
```

```rust,no_run,noplayground
{{#include ../code014/src/fundamentals/async_compute.rs:channels-app}}
```

Make sure to add `async_channel` to your `Cargo.toml`:

```toml
[dependencies]
async-channel = "2.3.1"
```

## Wider Async Ecosystem

Bevy's task pools are built on top of the [`smol`][project::smol] runtime.

Feel free to use anything from its ecosystem of compatible crates:

 - `async-channel` - Multi-producer multi-consumer channels
 - `async-fs` - Async filesystem primitives
 - `async-net` - Async networking primitives (TCP/UDP/Unix)
 - `async-process` - Async interface for working with processes
 - `async-lock` - Async locks (barrier, mutex, reader-writer lock, semaphore)
 - `async-io` - Async adapter for I/O types, also timers
 - `futures-lite` - Misc helper and extension APIs
 - `futures` - More helper and extension APIs
   (notably the powerful [`select!`][futures::select] and [`join!`][futures::join] macros)
 - Any Rust async library that supports [`smol`][project::smol].

## Using Your Own Threads

While not typically recommended, sometimes you might want to manage an
actual dedicated CPU thread of your own. For example, if you also want to run
another framework's runtime (such as [`tokio`][project::tokio]) in parallel
with Bevy. You might have to do this if you have to use crates built for
another async ecosystem, that are not compatible with [`smol`][project::smol].

To interoperate with your non-Bevy thread, you can move data between
it and Bevy using channels. Do the equivalent of what was shown [in
the example earlier on this page](#passing-data-around), but instead of
[`async-channel`][docs::async-channel], use the channel types provided
by your alternative runtime (such as [`tokio`][project::tokio]), or
[`std`][std::mpsc]/[`crossbeam`][docs::crossbeam] for raw OS threads.
