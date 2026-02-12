
// Module Declarations
mod states_game;
mod states_ui;
mod spawns;

// Imports
use bevy::prelude::*;
use crate::states_game::*;
use crate::states_ui::*;

fn main() {

    App::new()
        .add_plugins((DefaultPlugins, StatesForGame {}))
        .init_state::<GameState>()
        .init_resource::<LeftClickState>()
        .add_message::<LeftClick>()

        // Systems that run every frame.
        .add_systems(Update, (detect_left_click, test_click).chain())
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum UIState {
    #[default]
    MainMenu,
    Settings,
    GameBoardCreator,
    GameBoard,
    PauseMenu
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
