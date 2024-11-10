{{#include ../include/header014.md}}

# ECS Programming Introduction

This page will try to teach you the general ECS mindset/paradigm.

---

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

Also check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

ECS is a programming paradigm that separates data and behavior. Bevy will store
all of [your data][cb::ecs-intro-data] and manage all of [your individual pieces
of functionality][cb::ecs-intro-code] for you. The code will run when
appropriate. Your code can get access to whatever data it needs to do its thing.

This makes it easy to write game logic ([Systems][cb::system]) in a way that
is flexible and reusable. For example, you can implement:

- health and damage that works the same way for anything in the game,
  regardless of whether that's the player, an NPC, or a monster, or a vehicle
- gravity and collisions for anything that should have physics
- an animation or sound effect for all buttons in your UI

Of course, when you need specialized behavior only for specific entities (say,
player movement, which only applies to the player), that is naturally easy to
express, too.

If you are familiar with database programming, you will feel right at home. ECS
is conceptually very similar to a lightweight in-memory database.

[Read more about how to represent your data.][cb::ecs-intro-data]

[Read more about how to represent your functionality.][cb::ecs-intro-code]
