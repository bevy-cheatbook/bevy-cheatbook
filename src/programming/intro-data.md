{{#include ../include/header011.md}}

# Intro: Your Data

This page is an overview, to give you an idea of the big picture of how Bevy
works. Click on the various links to be taken to dedicated pages where you can
learn more about each concept.

---

As mentioned in [the ECS Intro][cb::ecs-intro], Bevy stores all your data for
you and allows you easy and flexible access to whatever you need, wherever you
need it.

The ECS's data structure is called the [`World`][bevy::World]. That is what
stores and manages all of the data. For advanced scenarios, is possible to have
[multiple worlds][cb::multi-world], and then each one will behave as its own
separate ECS. However, normally, you just work with the main World that Bevy
sets up for your [App][cb::app].

You can represent your data in two different ways:
[Entities/Components](#entities--components), and [Resources](#resources).

## Entities / Components

Conceptually, you can think of it by analogy with tables, like in a database or
spreadsheet. Your different data types ([Components][cb::component]) are like
the "columns" of a table, and there can be arbitrarily many "rows"
([Entities][cb::entity]) containing values / instances of various components.
The [`Entity`][bevy::Entity] ID is like the row number. It's an integer index
that lets you find specific component values.

Component types that are empty `struct`s (contain no data) are called [marker
components][cb::component-marker]. They are useful as "tags" to identify
specific entities, or enable certain behaviors. For example, you could use them
to identify the player entity, to mark enemies that are currently chasing the
player, to select entities to be despawned at the end of the level, etc.

Here is an illustration to help you visualize the logical structure. The
checkmarks show what component types are present on each entity. Empty cells
mean that the component is not present. In this example, we have a player,
a camera, and several enemies.

|[`Entity`][bevy::Entity] (ID)|[`Transform`][bevy::Transform]|`Player`|`Enemy`|[`Camera`][bevy::Camera]|`Health`|...|
|---|---|---|---|---|---|---|
|...|||||||
|107|✓ `<translation>` `<rotation>` `<scale>`|✓|||✓ `50.0`||
|108|✓ `<translation>` `<rotation>` `<scale>`||✓||✓ `25.0`||
|109|✓ `<translation>` `<rotation>` `<scale>`|||✓ `<camera data>`|||
|110|✓ `<translation>` `<rotation>` `<scale>`||✓||✓ `10.0`||
|111|✓ `<translation>` `<rotation>` `<scale>`||✓||✓ `25.0`||
|...|||||||

Representing things this way gives you flexibility. For example, you could
create a `Health` component for your game. You could then have many entities
representing different things in your game, such as the player, NPCs, or
monsters, all of which can have a `Health` value (as well as other relevant
components).

The typical and obvious pattern is to use entities to represent "objects in the
game/scene", such as the camera, the player, enemies, lights, props, UI
elements, and other things. However, you are not limited to that. The ECS is a
general-purpose data structure. You can create entities and components to store
any data. For example, you could create an entity to store a bunch of settings
or configuration parameters, or other abstract things.

Data stored using Entities and Components is accessed using [queries][cb::query].
For example, if you want to implement a game mechanic, write a [system][cb::system],
specify what component types you want to access, and do your thing. You can either
iterate through all entities that match your specification, or access specific
ones (if you know their [`Entity`][bevy::Entity] ID).

Bevy can automatically keep track of what data your [systems][cb::system] have
access to and [run them in parallel][cb::system-parallel] on multiple CPU
cores. This way, you get multithreading with no extra effort from you!

If you want to modify the data structure itself (as opposed to just accessing
existing data), such as to create or remove entities and components, that
requires special consideration. Bevy cannot change the memory layout while other
systems might be running. These operations can be buffered/deferred using
[Commands][cb::commands], to be applied later when it is safe to do so. You can
also get [direct World access][cb::world] using [exclusive
systems][cb::exclusive], if you want to perform such operations
immediately (but without multithreading).

### Comparison with Object-Oriented Programming

Object-Oriented programming teaches you to think of everything as "objects",
where each object is an instance of a "class". The class specifies the data
and functionality for all objects of that type, in one place. Every object
of that class has the same data (with different values) and the same
associated functionality.

This is the opposite of the ECS mentality. In ECS, any [entity][cb::entity] can
have any data (any combination of [components][cb::component]). The purpose of
entities is to identify that data. Your [systems][cb::system] are loose pieces
of functionality that can operate on any data. They can easily find what they
are looking for, and implement the desired behavior.

If you are an object-oriented programmer, you might be tempted to define a big
monolithic `struct Player` containing all the fields / properties of the player.
In Bevy, this is considered bad practice, because doing it that way can make it
more difficult to work with your data and limit performance. Instead, you should
make things granular, when different pieces of data may be accessed independently.

For example, represent the player in your game as an entity, composed
of separate component types (separate `struct`s) for things like the
health, XP, or whatever is relevant to your game. You can also attach
standard Bevy components like [`Transform`][bevy::Transform] ([transforms
explained][cb::transform]) to it.

Then, each piece of functionality (each [system][cb::system]) can just
query for the data it needs. Common functionality (like a health/damage
system) can be applied to any entity with the matching components,
regardless of whether that's the player or something else in the game.

However, if some data always makes sense to be accessed together, then you
should put it in a single `struct`. For example, Bevy's
[`Transform`][bevy::Transform] or [`Color`][bevy::Color]. With these types, the
fields are not likely to be useful independently.

### Additional Internal Details

The set / combination of components that a given entity has, is called the
entity's Archetype. Bevy keeps track of that internally, to organize the data in
RAM. Entities of the same Archetype have their data stored together, which
allows the CPU to access and cache it efficiently.

If you add/remove component types on existing entities, you are changing the
Archetype, which may require Bevy to copy the data to a different location.

[Learn more about Bevy's component storage.][cb::component-storage]

## Resources

If there is only one global instance (singleton) of something, and it is
standalone (not associated with other data), create a [Resource][cb::res].

For example, you could create a resource to store your game's graphics
settings, or the data for the currently active game mode or session.

This is a simple way of storing data, when you know you don't need the
flexibility of Entities/Components.
