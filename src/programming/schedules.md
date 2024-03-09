{{#include ../include/header013.md}}

# Schedules

See also: [ECS Intro: Your Code][cb::ecs-intro-code], for a general overview
of Bevy's scheduling framework.

---

All [systems][cb::system] to be run by Bevy are contained and organized using
schedules. A schedule is a collection of systems, with metadata for how they
should run, and an associated executor algorithm to run the systems.

A Bevy app has many different schedules for different purposes, to run them
in different situations.

## Scheduling Systems

If you want a [system][cb::system] to be run by Bevy, you need to add it to a
schedule via the [app builder][cb::app]. Writing a new Rust function and
forgetting to add it / register it with Bevy is a common mistake.

Whenever you add a system, you specify what schedule to put it in:

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:add-system}}
```

### Per-System Configuration

You can add metadata to your systems, to affect how they will be run.

This can include:
 - [Run Conditions][cb::rc] to control if a system should run
 - [Ordering Dependencies][cb::system-order], if a system should run before/after specific other systems in the same schedule
 - [System Sets][cb::systemset] to group systems together, so common configuration can be applied to all of them

When the schedule runs, the executor algorithm will honor all of this
configuration when determining if a system is ready to run. A system is ready
when all of the following is true:
 - No other currently-running system is accessing any of the same data mutably (as per the [system parameters][cb::system-param])
 - All of the systems [ordered][cb::system-order] "before" have finished or have been skipped due to run conditions
 - The system's [run conditions][cb::rc] all return true

When a system becomes ready, it will be run on an available CPU thread. Systems
run in a non-deterministic order by default! A system might run at different
times every frame. If you care about its relationship to other systems, add
[ordering dependencies][cb::system-order].

### Dynamically Adding/Removing Systems

Bevy's schedules do not (yet?) support adding and removing systems at runtime.
You need to configure everything ahead of time.

You should add all systems you might want to run, and then control them using
[run conditions][cb::rc]. That is the mechanism for disabling them if they
shouldn't run.

## Bevy's App Structure

Bevy has three primary/foundational schedules: [`Main`], [`Extract`], [`Render`].
There are also other schedules, which are managed and run within [`Main`].

In a normal Bevy app, the [`Main`]+[`Extract`]+[`Render`] schedules are run repeatedly
in a loop. Together, they produce one frame of your game. Every time [`Main`]
runs, it runs a sequence of other schedules. On its first run, it also first
runs a sequence of "startup" schedules.

Most Bevy users only have to deal with the sub-schedules of [`Main`].
[`Extract`] and [`Render`] are only relevant to graphics developers who want to
develop new/custom rendering features for the engine. This page is only focused
on [`Main`]. If you want to learn more about [`Extract`] and [`Render`], [see
this page about Bevy's rendering architecture][cb::render-architecture].

## The Main Schedule

[`Main`] is where all the application logic runs. It is a sort of meta-schedule,
whose job is to run other schedules in a specific order. You should not add any
custom systems directly to [`Main`]. You should add your systems to the various
schedules managed by [`Main`].

Bevy provides the following schedules, to organize all the systems:

 - [`First`], [`PreUpdate`], [`StateTransition`], [`RunFixedMainLoop`], [`Update`], [`PostUpdate`], [`Last`]
   - These schedules are run by [`Main`] every time it runs
 - [`PreStartup`], [`Startup`], [`PostStartup`]
   - These schedules are run by [`Main`] once, the first time it runs
 - [`FixedMain`]
   - The [fixed timestep][cb::fixedtimestep] equivalent of the [`Main`] schedule.
   - Run by [`RunFixedMainLoop`] as many times as needed, to catch up to the fixed timestep interval.
 - [`FixedFirst`], [`FixedPreUpdate`], [`FixedUpdate`], [`FixedPostUpdate`], [`FixedLast`]
   - The [fixed timestep][cb::fixedtimestep] equivalents of the [`Main`] sub-schedules.
 - [`OnEnter(…)`][`OnEnter`]/[`OnExit(…)`][`OnExit`]/[`OnTransition(…)`][`OnTransition`]
   - These schedules are run by [`StateTransition`] on [state][cb::state] changes

The intended places for most user systems (your game logic) are [`Update`],
[`FixedUpdate`], [`Startup`], and the [state][cb::state] transition schedules.

[`Update`] is for your usual game logic that should run every frame. [`Startup`] is
useful to perform initialization tasks, before the first normal frame update
loop. [`FixedUpdate`] is if you want to use a [fixed timestep][cb::fixedtimestep].

The other schedules are intended for engine-internal functionality. Splitting
them like that ensures that Bevy's internal engine systems will run correctly
with respect to your systems, without any configuration on your part.
Remember: Bevy's internals are implemented using ordinary systems
and ECS, just like your own stuff!

If you are developing plugins to be used by other people, you might be
interested in adding functionality to [`PreUpdate`]/[`PostUpdate`] (or the `Fixed`
equivalents), so it can run alongside other "engine systems". Also consider
[`PreStartup`] and [`PostStartup`] if you have startup systems that should be
separated from your users' startup systems.

[`First`] and [`Last`] exist only for special edge cases, if you really need to
ensure something runs before/after *everything* else, including all the normal
"engine-internal" code.

## Configuring Schedules

Bevy also offers some features that can be configured at the schedule level.

### Single-Threaded Schedules

If you consider multi-threading to not be working well for you, for whatever reason,
you can disable it per-schedule.

In a single-threaded schedule, systems will run one at a time, on the main
thread.  However, the same "readiness" algorithm is still applied and so
systems can run in an undefined order. You should still specify [ordering
dependencies][cb::system-order] where you need determinism.

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:single-threaded}}
```

### Ambiguity Detection

The Ambiguity Detector is an optional Bevy feature that can help you debug issues
related to non-determinism.

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:ambiguity-detector}}
```

It will print warnings for any combination of systems where at least one of
them accesses some piece of data ([resource][cb::res] or
[component][cb::component]) mutably, but the others don't have explicit
[ordering dependencies][cb::system-order] on that system.

Such situations might indicate a bug, because you don't know if the systems that
read the data would run before or after the system that mutates the data.

It is up to you to decide if you care about this, on a case-by-case basis.

### Deferred Application

Normally, Bevy will automatically manage where [Commands][cb::commands]
and other deferred operations get applied. If [systems][cb::system] have
[ordering dependencies][cb::system-order] on one another, Bevy will make
sure to apply any pending deferred operations from the first system before
the second system runs.

If you would like to disable this automatic behavior and manually manage
the sync points, you can do that.

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:disable-auto-apply-deferred}}
```

Now, to manually create sync points, add special [`apply_deferred`] systems
where you like them:

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:apply-deferred}}
```

## Main Schedule Configuration

The order of schedules to be run by [`Main`] every frame is configured in the
[`MainScheduleOrder`] [resource][cb::res]. For advanced use cases, if Bevy's
predefined schedules don't work for your needs, you can change it.

### Creating a New Custom Schedule

As an example, let's say we want to add an additional schedule, that runs every
frame (like [`Update`]), but runs before [fixed timestep][cb::fixedtimestep].

First, we need to create a name/label for our new schedule, by creating a Rust
type (a `struct` or `enum`) and deriving [`ScheduleLabel`] + an assortment of
required standard Rust traits.

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:custom-schedule}}
```

Now, we can init the schedule in the [app][cb::app], add it to
[`MainScheduleOrder`] to make it run every frame where we like it, and add some
systems to it!

```rust,no_run,noplayground
{{#include ../code013/src/programming/schedules.rs:custom-schedule-app}}
```
