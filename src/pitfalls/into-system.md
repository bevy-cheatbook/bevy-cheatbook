{{#include ../include/header010.md}}

# Obscure Rust compiler errors

You can get confusing compiler errors when you try to add [systems][cb::system]
to your Bevy [app][cb::app].

## Common beginner mistakes

  - Using `commands: &mut Commands` instead of `mut commands: Commands`.
  - Using `Query<MyStuff>` instead of `Query<&MyStuff>` or `Query<&mut MyStuff>`.
  - Using `Query<&ComponentA, &ComponentB>` instead of `Query<(&ComponentA, &ComponentB)>`
    (forgetting the tuple)
  - Using your resource types directly without [`Res`][bevy::Res] or [`ResMut`][bevy::ResMut].
  - Using your component types directly without putting them in a [`Query`][bevy::Query].
  - Using other arbitrary types in your function.

Note that `Query<Entity>` is correct, because the Entity ID is special;
it is not a component.

## Error adding function as system

The errors can look like this:

```
error[E0277]: the trait bound `for<'a, 'b, 'c> fn(...) {system}: IntoSystem<(), (), _>` is not satisfied
   --> src/main.rs:5:21
    |
5   |         .add_system(my_system)
    |          ---------- ^^^^^^^^^ the trait `IntoSystem<(), (), _>` is not implemented for fn item `for<'a, 'b, 'c> fn(...) {system}`
    |          |
    |          required by a bound introduced by this call
    |
    = help: the trait `IntoSystemAppConfig<()>` is implemented for `SystemAppConfig`
    = note: required for `for<'a, 'b, 'c> fn(...) {system}` to implement `IntoSystemConfig<_>`
    = note: required for `for<'a, 'b, 'c> fn(...) {system}` to implement `IntoSystemAppConfig<_>`
```

The error (confusingly) points to the place in your code where you try to add the system,
but in reality, the problem is actually in the `fn` function definition!

This is caused by your function having invalid parameters. [Bevy can
only accept special types as system parameters!][builtins::systemparam]

## Error on malformed queries

You might also errors that look like this:

```
error[E0277]: the trait bound `Transform: WorldQuery` is not satisfied
   --> src/main.rs:10:12
    |
10  |     query: Query<Transform>,
    |            ^^^^^^^^^^^^^^^ the trait `WorldQuery` is not implemented for `Transform`
    |
    = help: the following other types implement trait `WorldQuery`:
              &'__w mut T
              &T
              ()
              (F0, F1)
              (F0, F1, F2)
              (F0, F1, F2, F3)
              (F0, F1, F2, F3, F4)
              (F0, F1, F2, F3, F4, F5)
            and 54 others
note: required by a bound in `bevy::prelude::Query`
   --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.10.0/src/system/query.rs:276:37
    |
276 | pub struct Query<'world, 'state, Q: WorldQuery, F: ReadOnlyWorldQuery = ()> {
    |                                     ^^^^^^^^^^ required by this bound in `Query`
```

To access your components, you need to use reference syntax (`&` or `&mut`).

```
error[E0107]: struct takes at most 2 generic arguments but 3 generic arguments were supplied
   --> src/main.rs:10:12
    |
10  |     query: Query<&Transform, &Camera, &GlobalTransform>,
    |            ^^^^^                      ---------------- help: remove this generic argument
    |            |
    |            expected at most 2 generic arguments
    |
note: struct defined here, with at most 2 generic parameters: `Q`, `F`
   --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.10.0/src/system/query.rs:276:12
    |
276 | pub struct Query<'world, 'state, Q: WorldQuery, F: ReadOnlyWorldQuery = ()> {
    |            ^^^^^                 -              --------------------------
```

When you want to query for multiple components, you need to put them in a tuple:
`(&Transform, &Camera, &GlobalTransform)`.
