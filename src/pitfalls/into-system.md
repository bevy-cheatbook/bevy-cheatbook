# Error adding function as system

{{#include ../include/links.md}}

You can sometimes get confusing arcane compiler errors when you try to add
systems to your Bevy app.

The errors can look like this:

```
the trait bound `for<'r, 's, 't0> fn(bevy::prelude::Query<'r, 's, (&'t0 Param)) {my_system}: IntoSystem<(), (), _>` is not satisfied
```

This is caused by your function having incompatible parameters. Bevy can
only accept special types as system parameters.

You might also errors that look like this:

```
the trait bound `Component: WorldQuery` is not satisfied
the trait `WorldQuery` is not implemented for `Component`
```

```
this struct takes at most 2 type arguments but 3 type arguments were supplied
```

These errors are caused by a malformed query.

## Common beginner mistakes

  - Using `&mut Commands` (bevy 0.4 syntax) instead of [`Commands`][bevy::Commands].
  - Using `Query<MyStuff>` instead of `Query<&MyStuff>` or `Query<&mut MyStuff>`.
  - Using `Query<&ComponentA, &ComponentB>` instead of `Query<(&ComponentA, &ComponentB)>`
    (forgetting the tuple)
  - Using your resource types directly without [`Res`][bevy::Res] or [`ResMut`][bevy::ResMut].
  - Using your component types directly without putting them in a [`Query`][bevy::Query].
  - Using other arbitrary types in your function.

Note that `Query<Entity>` is correct, because the Entity ID is special;
it is not a component.

## Supported types

Only the following types are supported as system parameters:

{{#include ../include/builtins.md:systemparams}}
