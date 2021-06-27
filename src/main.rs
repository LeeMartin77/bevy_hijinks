mod components;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::systems::orbital_plugin::OrbitalPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(OrbitalPlugin)
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .run();
}