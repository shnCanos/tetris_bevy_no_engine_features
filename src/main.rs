use bevy::prelude::*;
mod game;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(game::GamePlugin)
    .run();
}
