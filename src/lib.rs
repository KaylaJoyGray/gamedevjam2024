mod utils;
mod gfx;
mod sound;

use wasm_bindgen::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("map.tmx");

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

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
