//TODO can these be in one line?
mod jordquest;
mod input;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, jordquest::JordQuestPlugin))
        .run();
}


