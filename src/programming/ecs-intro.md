# ECS as a Data Structure

Relevant official examples:
[`ecs_guide`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs).

---

Bevy stores and manages all your data for you, using the Bevy ECS
(Entity-Component System).

Conceptually, you can think of it by analogy with tables, like in a database
or spreadsheet. Your different data types (Components) are like the "columns"
of a table, and there can be arbitrarily many "rows" (Entities) containing
values / instances of each component.

For example, you could create a `Health` component for your game. You could
then have many entities representing different things in your game, such
as the player, NPCs, or monsters, all of which can have a `Health` value
(as well as other relevant components).

This makes it easy to write game logic ([Systems](./systems.md)) that can
operate on any entity with the necessary components (such as a health/damage
system for anything that has `Health`), regardless of whether that's the
player, an NPC, or a monster (or anything else). This makes your game logic
very flexible and reusable.

The set / combination of components that a given entity has, is called the
entity's Archetype.

Note that entities aren't limited to just "objects in the game world". The ECS
is a general-purpose data structure. You can create entities and components
to store any data.

## Performance

Bevy has a smart scheduling algorithm that runs your systems in parallel
as much as possible. It does that automatically, when your functions don't
require conflicting access to the same data. Your game will scale to run on
multiple CPU cores "for free"; that is, without requiring extra development
effort from you.

To improve the chances for parallelism, you can make your data and code more
granular. Split your data into smaller types / `struct`s. Split your logic
into multiple smaller systems / functions. Have each system access only the
data that is relevant to it. The fewer access conflicts, the faster your
game will run.

The general rule of thumb for Bevy performance is: more granular is better.

## Note for Programmers coming from Object-Oriented Languages

You may be used to thinking in terms of "object classes". For example, you
might be tempted to define a big monolithic `struct Player` containing all
the fields / properties of the player.

In Bevy, this is considered bad practice, because doing it that way can make
it more difficult to work with your data, and limit performance.

Instead, you should make things granular, when different pieces of data may
be accessed independently.

For example, represent the Player in your game as an entity, composed of
separate component types (separate `struct`s) for things like the health,
XP, or whatever is relevant to your game. You can also attach standard Bevy
components like `Transform` to it.

This will make it easier for you to develop your systems (game logic /
behaviors), as well as make your game's runtime performance better.

However, something like a `Transform`, or a set of coordinates, still makes
sense as a single `struct`, because its fields are not likely to be useful
independently.
