{{#include ../include/header014.md}}

# Obscure Rust compiler errors

You can get scary-looking compiler errors when you try to add [systems][cb::system]
to your Bevy [app][cb::app].

## Common beginner mistakes

  - Using `commands: &mut Commands` instead of `mut commands: Commands`.
  - Using `Query<MyStuff>` instead of `Query<&MyStuff>` or `Query<&mut MyStuff>`.
  - Using `Query<&ComponentA, &ComponentB>` instead of `Query<(&ComponentA, &ComponentB)>`
    (forgetting the tuple)
  - Using your [resource][cb::res] types directly without [`Res`] or [`ResMut`].
  - Using your [component][cb::component] types directly without putting them in a [`Query`].
  - Using a [bundle][cb::bundle] type in a [query][cb::query]. You want individual components.
  - Using other arbitrary types in your function.

Note that `Query<Entity>` is correct, because the Entity ID is special;
it is not a component.

## Error adding function as system

The errors can look like this:

```
error[E0277]: `for<'a, 'b, 'c> fn(…) {my_system}` does not describe a valid system configuration
   --> src/main.rs:11:30
    |
11  |         .add_systems(Update, my_system)
    |          -----------         ^^^^^^^^^ invalid system configuration
    |          |
    |          required by a bound introduced by this call
    |
    = help: the trait `IntoSystem<(), (), _>` is not implemented for fn item `for<'a, 'b, 'c> fn(…) {my_system}`, which is required by `for<'a, 'b, 'c> fn(…) {my_system}: IntoSystemConfigs<_>`
    = help: the following other types implement trait `IntoSystemConfigs<Marker>`:
              <(S0, S1) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1)>>
              <(S0, S1, S2) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2)>>
              <(S0, S1, S2, S3) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2, P3)>>
              <(S0, S1, S2, S3, S4) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2, P3, P4)>>
              <(S0, S1, S2, S3, S4, S5) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2, P3, P4, P5)>>
              <(S0, S1, S2, S3, S4, S5, S6) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2, P3, P4, P5, P6)>>
              <(S0, S1, S2, S3, S4, S5, S6, S7) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2, P3, P4, P5, P6, P7)>>
              <(S0, S1, S2, S3, S4, S5, S6, S7, S8) as IntoSystemConfigs<(SystemConfigTupleMarker, P0, P1, P2, P3, P4, P5, P6, P7, P8)>>
            and 14 others
    = note: required for `for<'a, 'b, 'c> fn(…) {my_system}` to implement `IntoSystemConfigs<_>`
note: required by a bound in `bevy::prelude::App::add_systems`
   --> /home/iyes/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_app-0.14.0-rc.2/src/app.rs:287:23
    |
284 |     pub fn add_systems<M>(
    |            ----------- required by a bound in this associated function
...
287 |         systems: impl IntoSystemConfigs<M>,
    |                       ^^^^^^^^^^^^^^^^^^^^ required by this bound in `App::add_systems`
```

The error (confusingly) points to the place in your code where you try to add the system,
but in reality, the problem is actually in the `fn` function definition!

This is caused by your function having invalid parameters. [Bevy can
only accept special types as system parameters!][builtins::systemparam]

## Error on malformed queries

You might also errors that look like this:

```
error[E0277]: `bevy::prelude::AnimationPlayer` is not valid to request as data in a `Query`
   --> src/main.rs:60:18
    |
60  |     mut players: Query<AnimationPlayer, &Transform>,
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid `Query` data
    |
    = help: the trait `QueryData` is not implemented for `bevy::prelude::AnimationPlayer`
    = help: the following other types implement trait `QueryData`:
              &'__w mut T
              &Archetype
              &T
              ()
              (F0, F1)
              (F0, F1, F2)
              (F0, F1, F2, F3)
              (F0, F1, F2, F3, F4)
            and 41 others
note: required by a bound in `bevy::prelude::Query`
   --> /home/iyes/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.14.0-rc.2/src/system/query.rs:349:37
    |
349 | pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    |                                     ^^^^^^^^^ required by this bound in `Query`

error[E0277]: `&bevy::prelude::Transform` is not a valid `Query` filter
   --> src/main.rs:60:18
    |
60  |     mut query: Query<AnimationPlayer, &Transform>,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid `Query` filter
    |
    = help: the trait `QueryFilter` is not implemented for `&bevy::prelude::Transform`
    = note: a `QueryFilter` typically uses a combination of `With<T>` and `Without<T>` statements
    = help: the following other types implement trait `QueryFilter`:
              ()
              (F0, F1)
              (F0, F1, F2)
              (F0, F1, F2, F3)
              (F0, F1, F2, F3, F4)
              (F0, F1, F2, F3, F4, F5)
              (F0, F1, F2, F3, F4, F5, F6)
              (F0, F1, F2, F3, F4, F5, F6, F7)
            and 28 others
note: required by a bound in `bevy::prelude::Query`
   --> /home/iyes/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.14.0-rc.2/src/system/query.rs:349:51
    |
349 | pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    |                                                   ^^^^^^^^^^^ required by this bound in `Query`

error[E0107]: struct takes at most 2 generic arguments but 3 generic arguments were supplied
   --> src/main.rs:60:18
    |
60  |     mut query: Query<AnimationPlayer, &Transform, &mut GlobalTransform>,
    |                ^^^^^                              -------------------- help: remove this generic argument
    |                |
    |                expected at most 2 generic arguments
    |
note: struct defined here, with at most 2 generic parameters: `D`, `F`
   --> /home/iyes/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.14.0-rc.2/src/system/query.rs:349:12
    |
349 | pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    |            ^^^^^                 -             -------------------
```

To access your components, you need to use reference syntax (`&` or `&mut`).

When you want to query for multiple components, you need to put them in a tuple:
`Query<(&mut Transform, &Camera, &MyComponent)>`.
