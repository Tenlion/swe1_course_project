
// Module Declarations
mod states_game;
mod states_ui;
mod spawns;
mod resources;

// Imports
use bevy::prelude::*;
use crate::resources::GameResources;
use crate::states_game::StatesForGame;
use crate::states_ui::StatesForUI;
use crate::spawns::Spawns;

fn main() {

    App::new()

        // Plugins!  Which are also just files that we make and can add to the game in order
        // to condense and organize code for projects within Bevy.
        .add_plugins(DefaultPlugins)
        .add_plugins(GameResources{})
        .add_plugins(StatesForUI{})
        .add_plugins(StatesForGame{})
        .add_plugins(Spawns{})

        // Tells the app to launch.
        .run();
}
