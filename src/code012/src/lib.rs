#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_code)]

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
}
mod d2 {
}
mod d3 {
}
mod input {
    mod mouse;
    mod char;
}
mod window {
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
mod platforms {
}

// cookbook chapter uses `examples`, they should be runnable
