## New Cargo Resolver

Cargo recently added a new dependency resolver algorithm, that is incompatible
with the old one. Bevy *requires* the new resolver.

If you are just creating a new blank Cargo project, don't worry. This should
already be setup correctly by `cargo new`.

If you are getting weird compiler errors from Bevy dependencies, read on. Make sure
you have the correct configuration, and then [clear the cargo state](/pitfalls/build-errors.md#clear-the-cargo-state).

### Single-Crate Projects

In a single-crate project (if you only have one `Cargo.toml` file in your project),
if you are using the latest Rust2021 Edition, the new resolver is automatically
enabled.

So, you need either one of these settings in your `Cargo.toml`:

```toml
[package]
edition = "2021"
```

or

```toml
[package]
resolver = "2"
```

### Multi-Crate Workspaces

In a multi-crate Cargo workspace, the resolver is a global setting for the
whole workspace. It will *not* be enabled by default.

This can bite you if you are transitioning a single-crate project into a workspace.

You *must* add it manually to the top-level `Cargo.toml` for your Cargo Workspace:

```toml
[workspace]
resolver = "2"
```
