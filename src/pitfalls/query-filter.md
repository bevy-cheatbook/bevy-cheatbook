# Combining Query Filters

[Query Filters](../programming/queries.md#query-filters) like
`With<T>`/`Without<T>` take a single component type `T`. If you want to
filter based on multiple components, you can create a `Query` that has
multiple filters in a tuple:

```rust,no_run,noplayground
Query<&Stuff, (With<ComponentA>, With<ComponentB>)>
```

It is a common beginner mistake to instead put multiple components in a
tuple inside a single filter. This is *incorrect*:

```rust,no_run,noplayground
Query<&Stuff, With<(ComponentA, ComponentB)>>
```

The above would compile and run, but treat the whole tuple as a single
component type. Since it is looking for entities that have the tuple as a
component, it will not give the correct results as you would expect.
