# Error adding function as system

You can sometimes get confusing arcane compiler errors when you try to add
systems to your Bevy app.

The errors can look like this:

```
no method named `system` found for fn item `for<'r, 's> fn(bevy::prelude::Query<'r, &'s Component>, bevy::prelude::Commands) {my_system}` in the current scope
`my_system` is a function, perhaps you wish to call it
```

This is caused by your function having incompatible parameters. Bevy can only
accept special types as system parameters.

You might also get an error that looks like this:

```
the trait bound `Component: WorldQuery` is not satisfied
the trait `WorldQuery` is not implemented for `Component`
```

This is caused by a malformed query, such as if you write `Query<Component>` instead of `Query<&Component>` or `Query<&mut Component>`.

## Common beginner mistakes

1. Using `Commands` instead of `&mut Commands`.
2. Using `Query<MyStuff>` instead of `Query<&MyStuff>` or `Query<&mut MyStuff>`.
3. Using your resource types directly without `Res` or `ResMut`.
4. Using your component types directly without putting them in a `Query`.
5. Using other arbitrary types in your function.

Note that `Query<Entity>` is correct, because the Entity ID is special; it is not a component.

## Supported types (bevy 0.4)

It can be difficult to figure out what types are supported from the [API
docs](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.FetchSystemParam.html), so I
will give you a nice summary here on this page.

Only the following types are supported as system parameters:

{{#include ../include/systemparams-release.md}}

You can nest tuples as much as you want, to avoid running into the limits on the
maximum numbers of parameters, or simply to organize your parameters into groups.

## Supported types (bevy git)

{{#include ../include/systemparams-master.md}}
