 - `Commands`
 - `Res<T>` / `ResMut<T>`
 - `Option<Res<T>>` / `Option<ResMut<T>>`
 - `Local<T>`
 - `EventReader<T>`
 - `EventWriter<T>`
 - `Query<T, F = ()>`; can contain tuples of up to 15 types
 - `QuerySet` with up to 4 queries
 - `NonSend<T>` / `NonSendMut<T>`
 - `Entities`
 - `Components`
 - `Bundles`
 - `Archetypes`
 - `RemovedComponents<T>`
 - `Arc<parking_lot::Mutex<Commands>>`
 - `DrawContext`
 - tuples containing any of these types, with up to 16 members
 
Your function can have a maximum of 16 total parameters. If you need more, group
them into tuples to work around the limit.
