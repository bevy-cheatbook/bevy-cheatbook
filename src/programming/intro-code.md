{{#include ../include/header014.md}}

# Intro: Your Code

This page is an overview, to give you an idea of the big picture of how Bevy
works. Click on the various links to be taken to dedicated pages where you can
learn more about each concept.

---

As mentioned in [the ECS Intro][cb::ecs-intro], Bevy manages all of your
functionality/behaviors for you, running them when appropriate and giving them
access to whatever parts of [your data][cb::ecs-intro-data] they need.

Individual pieces of functionality are called [systems][cb::system]. Each system
is a Rust function (`fn`) you write, which accepts [special parameter
types][cb::system-param] to indicate what [data][cb::ecs-intro-data] it needs to
access. Think of the function signature as a "specification" for what to fetch
from the ECS [`World`].

Here is what a [system][cb::system] might look like. Note how, just by looking
at the function parameters, we know *exactly* what [data][cb::ecs-intro-data]
can be accessed.

```rust,no_run,noplayground
{{#include ../code014/src/programming/intro_code.rs:example-system}}
```

(learn more about: [systems][cb::system], [queries][cb::query], [commands][cb::commands], [resources][cb::res], [entities][cb::entity], [components][cb::component])

## Parallel Systems

Based on the [parameter][cb::system-param] types of the [systems][cb::system]
you write, Bevy knows what data each system can access and whether it conflicts
with any other systems. Systems that do not conflict (don't access any of the
same data mutably) will automatically be [run in parallel][cb::system-parallel]
on different CPU threads. This way, you get multithreading, utilizing modern
multi-core CPU hardware effectively, with no extra effort from you!

For best parallelism, it is recommended that you keep your functionality and
[your data][cb::ecs-intro-data] granular. Split up your systems, so each
one has a narrowly-scoped purpose and access to only the data it needs. This
gives Bevy more opportunities for parallelism. Putting too much functionality
in one system, or too much data in a single [component][cb::component] or
[resource][cb::res] `struct`, limits parallelism.

Bevy's parallelism is non-deterministic by default. Your systems might run in a
different and unpredictable order relative to one another, unless you add
[ordering][cb::system-order] dependencies to constrain it.

## Exclusive Systems

[Exclusive][cb::exclusive] systems provide you with a way to get [full direct
access][cb::world] to the ECS [`World`][cb::World]. They cannot run in parallel
with other systems, because they can access anything and do anything. Sometimes,
you might need this additional power.

```rust,no_run,noplayground
{{#include ../code014/src/programming/intro_code.rs:exclusive}}
```

## Schedules

Bevy stores systems inside of [schedules][cb::schedule]
([`Schedule`]). The schedule contains the systems and all
relevant metadata to organize them, telling Bevy when and how to run them. Bevy
[Apps][cb::App] typically contain many schedules. Each one is a collection of
systems to be invoked in different scenarios (every frame update, [fixed
timestep][cb::fixedtimestep] update, at app startup, on [state][cb::state]
transitions, etc.).

The metadata stored in schedules allows you to control how systems run:
 - Add [run conditions][cb::rc] to control if systems should run during an
   invocation of the schedule. You can disable systems if you only need them
   to run sometimes.
 - Add [ordering][cb::system-order] constraints, if one system depends on
   another system completing before it.

Within schedules, systems can be grouped into [sets][cb::systemset]. Sets
allow multiple systems to share common configuration/metadata. Systems
inherit configuration from all sets they belong to. Sets can also inherit
configuration from other sets.

Here is an illustration to help you visualize the logical structure of a
schedule. Let's look at how a hypothetical "Update" (run every frame) schedule of a
game might be organized.

List of [systems][cb::system]:

|[System][cb::system] name|[Sets][cb::systemset] it belongs to|[Run conditions][cb::rc]|[Ordering constraints][cb::system-order]|
|---|---|---|---|
|`footstep_sound`|`AudioSet` `GameplaySet`||`after(player_movement)` `after(enemy_movement)`|
|`player_movement`|`GameplaySet`|`player_alive` `not(cutscene)`|`after(InputSet)`|
|`camera_movement`|`GameplaySet`||`after(InputSet)`|
|`enemy_movement`|`EnemyAiSet`|||
|`enemy_spawn`|`EnemyAiSet`|||
|`enemy_despawn`|`EnemyAiSet`||`before(enemy_spawn)`|
|`mouse_input`|`InputSet`|`mouse_enabled`||
|`controller_input`|`InputSet`|`gamepad_enabled`||
|`background_music`|`AudioSet`|||
|`ui_button_animate`||||
|`menu_logo_animate`|`MainMenuSet`|||
|`menu_button_sound`|`MainMenuSet` `AudioSet`|||
|...||||

List of [sets][cb::systemset]:

|[Set][cb::systemset] name|Parent Sets|[Run conditions][cb::rc]|[Ordering constraints][cb::system-order]|
|---|---|---|---|
|`MainMenuSet`||`in_state(MainMenu)`||
|`GameplaySet`||`in_state(InGame)`||
|`InputSet`|`GameplaySet`|||
|`EnemyAiSet`|`GameplaySet`|`not(cutscene)`|`after(player_movement)`|
|`AudioSet`||`not(audio_muted)`||

Note that it doesn't matter in what order systems are listed in the schedule.
Their [order][cb::system-order] of execution is determined by the metadata. Bevy
will respect those constraints, but otherwise run systems in parallel as much as
it can, depending on what CPU threads are available.

Also note how our hypothetical game is implemented using many individually-small
systems. For example, instead of playing audio inside of the `player_movement`
system, we made a separate `play_footstep_sounds` system. These two pieces of
functionality probably need to access different [data][cb::ecs-intro-data], so
putting them in separate systems allows Bevy more opportunities for parallelism.
By being separate systems, they can also have different configuration. The
`play_footstep_sounds` system can be added to an `AudioSet`
[set][cb::systemset], from which it inherits a `not(audio_muted)` [run
condition][cb::rc].

Similarly, we put mouse and controller input in separate systems. The `InputSet`
set allows systems like `player_movement` to share an ordering dependency
on all of them at once.

You can see how Bevy's scheduling APIs give you a lot of flexibility to organize
all the functionality in your game. What will you do with all this power? ;)

---

Here is how [schedule][cb::schedule] that was illustrated above could be
created in code:

```rust,no_run,noplayground
{{#include ../code014/src/programming/intro_code.rs:example-scheduling}}
```

(learn more about: [schedules][cb::schedule], [system sets][cb::systemset], [states][cb::state], [run conditions][cb::rc], [system ordering][cb::system-order])
