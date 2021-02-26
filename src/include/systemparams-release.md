 - `&mut Commands`
 - `Res<T>` / `ResMut<T>`
 - `Local<T>`
 - `Query<T, F = ()>`; `T` can be a tuple of up to 16 types
 - `QuerySet` with up to 4 queries
 - `ChangedRes<T>`
 - `Arc<parking_lot::Mutex<Commands>>`
 - `Or<(...)>` (use with `ChangedRes`), with up to 16 members
 - tuples containing any of the above, with up to 16 members
 - `DrawContext` (advanced usage, for rendering)
 
Your function can have a maximum of 16 total parameters.
