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

## Using Your Own Threads

While not typically recommended, sometimes you might want to manage an
actual dedicated CPU thread of your own. For example, if you also want to run
another framework (such as [`tokio`][project::tokio]) in parallel with Bevy.

To interoperate with your non-Bevy thread, you can move data between it
and Bevy using channels. Set up some channels and put the side you want
to access from Bevy in a [resource][cb::res]. To receive data from Bevy,
you should poll the channels using a non-blocking method, like `try_recv`,
to check if data is available.
