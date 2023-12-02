#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

// MUST MIRROR BOOK CHAPTER STRUCTURE / FILENAMES!

mod setup {
    mod perf;
}
mod pitfalls {
    mod d3_not_rendering;
    mod uv_coordinates;
}
mod fundamentals {
    mod transforms;
    mod visibility;
    mod log;
}
mod graphics {
    mod camera;
    mod hdr_tonemap;
    mod bloom;
}
mod d2 {
    mod camera;
}
mod d3 {
    mod camera;
}
mod input {
    mod mouse;
    mod char;
}
mod window {
    mod mouse_grab;
}
mod assets {
}
mod audio {
}
mod ui {
}
mod programming {
    mod intro_data;
    mod intro_code;
    mod app_builder;
    mod systems;
    mod res;
    mod ec;
    mod bundle;
    mod events;
    mod paramset;
    mod non_send;
}
mod gpu {
}
mod patterns {
}
mod cookbook {
    mod print_framerate;
    mod cursor2world;
}
mod platforms {
    mod wasm;
}
