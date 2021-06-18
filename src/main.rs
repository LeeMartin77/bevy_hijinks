use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        //.add_startup_system(add_people.system())
        //.add_system(hello_world.system())
        .run();
}