# Borrow multiple fields from struct

When you have a component or resource, that is larger struct with multiple
fields, sometimes you want to borrow several of the fields at the same time,
possibly mutably. This can result in a compiler error about conflicting borrows:

```rust,no_run,noplayground
struct MyThing {
    a: Foo,
    b: Bar,
}

fn helper_func(foo: &Foo, bar: &mut Bar) {
    // do something
}

fn my_system(mut q: Query<&mut MyThing>) {
    for thing in q.iter_mut() {
        helper_func(&thing.a, &mut thing.b); // ERROR!
    }
}
```

The error:

```
error[E0502]: cannot borrow `thing` as mutable because it is also borrowed as immutable
    |
    |         helper_func(&thing.a, &mut thing.b); // ERROR!
    |         -----------  -----         ^^^^^ mutable borrow occurs here
    |         |            |
    |         |            immutable borrow occurs here
    |         immutable borrow later used by call
```

The solution is to use the "reborrow" idiom, a common but non-obvious trick in Rust programming:

```rust,no_run,noplayground
        // add this at the start of the for loop, before using `thing`:
        let thing = &mut *thing;
```

Note that this line triggers [change detection](../programming/change-detection.md).
Even if you don't modify the data afterwards, the component gets marked as changed.

## Explanation

Bevy typically gives you access to your data via special wrapper types
(like `Res<T>`, `ResMut<T>`, and `Mut<T>` (when querying for components
mutably)). This lets Bevy track access to the data.

These are "smart pointer" types that use the Rust `Deref` trait to dereference
to your data. They usually work seamlessly and you don't even notice them.

However, in a sense, they are opaque to the compiler. The Rust language
allows fields of a struct to be borrowed individually, when you have direct
access to the struct, but this does not work when it is wrapped in another type.

The "reborrow" trick shown above, effectively converts the wrapper into a regular
Rust reference. `*thing` dereferences the wrapper via `Deref`, and then `&mut`
borrows it mutably. You now have `&mut MyStuff` instead of `Mut<MyStuff>`.
