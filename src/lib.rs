mod utils;
mod gfx;
mod sound;

use bevy::app::App;
use bevy::DefaultPlugins;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            gfx::GFXPlugin,
            sound::SoundPlugin,
        ))
        .run()
}
