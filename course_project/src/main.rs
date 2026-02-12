
// Module Declarations
mod states_game;
mod states_gui;
mod spawns;

// Imports
use bevy::prelude::*;
use crate::states_game::*;
use crate::states_gui::*;

fn main() {

    App::new()

        // Plugins!  Which are also just files that we make and can add to the game in order
        // to condense and organize code for projects within Bevy.
        .add_plugins(DefaultPlugins)
        .add_plugins(StatesForGame{})
        .add_plugins(StatesForGUI{})

        // Testing out custom states and how to create listeners through the message system
        // that Bevy throws to its scheduling service for events to trigger from.
        .init_resource::<LeftClickState>()
        .add_message::<LeftClick>()

        // Systems that run every frame.
        .add_systems(Update, (detect_left_click, test_click).chain())

        // Tells the app to launch.
        .run();
}

#[derive(Message)]
pub struct LeftClick {}

#[derive(Resource)]
pub struct LeftClickState {
    left_click_occurred: bool
}

impl Default for LeftClickState {
    fn default() -> Self {
        Self {
            left_click_occurred: false
        }
    }
}


pub fn detect_left_click (
    input: Res<ButtonInput<MouseButton>>,
    mut left_click_state: ResMut<LeftClickState>,
    mut left_click_event: MessageWriter<LeftClick>
)
{
    if input.just_pressed(MouseButton::Left) {
        left_click_event.write(LeftClick {});
        left_click_state.left_click_occurred = true;
    }
}

pub fn test_click (
    mut left_click_state: ResMut<LeftClickState>,
    mut game_state: ResMut<NextState<GameState>>
)
    -> Result<()>
{
    if left_click_state.left_click_occurred == true {
        println!("Test");
        left_click_state.left_click_occurred = false;
        game_state.set(GameState::Settings);
    }

    Ok(())
}
