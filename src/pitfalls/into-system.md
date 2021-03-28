# Error adding function as system

You can sometimes get confusing arcane compiler errors when you try to add
systems to your Bevy app.

The errors can look like this:

```
no method named `system` found for fn item `for<'r, 's> fn(...) {my_system}` in the current scope
`my_system` is a function, perhaps you wish to call it
```

This is caused by your function having incompatible parameters. Bevy can only
accept special types as system parameters.

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

 - Using `&mut Commands` (bevy 0.4 syntax) instead of `Commands`.
 - Using `Query<MyStuff>` instead of `Query<&MyStuff>` or `Query<&mut MyStuff>`.
 - Using `Query<&ComponentA, &ComponentB>` instead of `Query<(&ComponentA, &ComponentB)>` (forgetting the tuple)
 - Using your resource types directly without `Res` or `ResMut`.
 - Using your component types directly without putting them in a `Query`.
 - Using other arbitrary types in your function.

Note that `Query<Entity>` is correct, because the Entity ID is special; it is not a component.

## Supported types

It can be difficult to figure out what types are supported from the [API
docs](https://docs.rs/bevy/0.5.0/bevy/ecs/trait.SystemParam.html), so here is a list:

Only the following types are supported as system parameters:

{{#include ../include/systemparams.md}}

Note: the tuple/parameter limit used to be 16 (in Bevy 0.4). It was reduced due
to limitations from the Rust standard library.

You can nest tuples as much as you want, to avoid running into the limits on the
maximum numbers of parameters, or simply to organize your parameters into groups.
