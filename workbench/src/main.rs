use bevy::prelude::*;

fn main() {
    fn update() {}

    App::new()
        .add_systems(Update, update)
        .run();
}
