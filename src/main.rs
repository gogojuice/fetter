use bevy::prelude::*;

use fetter::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin))
    .run();
}
