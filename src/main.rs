mod jordquest;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, jordquest::JordQuestPlugin))
        .run();
}


