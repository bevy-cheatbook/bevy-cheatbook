{{#include ../include/header013.md}}

# Run Conditions

Run Conditions (RC) are a mechanism for controlling if Bevy should run specific
[systems][cb::system], at runtime. This allows you to enable/disable systems
on-demand, so that they only run sometimes.

RCs are Rust functions that return a value of type `bool`. They can accept
any [system parameters][builtins::systemparam], like a normal system, but
they must all be read-only (immutable).

```rust,no_run,noplayground
{{#include ../code013/src/programming/rc.rs:fn}}
```

RCs can be applied to individual [systems][cb::system] or to entire [system
sets][cb::systemset].

```rust,no_run,noplayground
{{#include ../code013/src/programming/rc.rs:app}}
```

When applied to a single [system][cb::system], Bevy will evaluate the RC at
the last moment, right before the system would otherwise be ready to run. If
you add the same RC to multiple systems, Bevy will evaluate it separately
for each one.

When applied to a [set][cb::systemset], the run condition will only be
evaluated once, before Bevy runs any system from the set, and if it returns
false, the entire set will be skipped.

Any given system can be governed by any number of RCs. You can add multiple RCs
to one system, and it will also inherit the RCs of any [sets][cb::systemset]
it belongs to. Bevy will evaluate all the RCs, and the system will only run
if all of them return `true`.

## Known Pitfalls

When receiving [events][cb::event] in systems that don't run every frame
update, you can miss some events that are sent while the receiving systems
are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.
