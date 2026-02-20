
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::spawns::*;

pub struct StatesForGUI {}
impl Plugin for StatesForGUI {
    fn build(&self, app: &mut App) {

        // Initializing a state type into the application.
        app.init_state::<GUIState>();

        // UI State Enter and Exit Definitions
        // These "states" are the GUI elements that players engage with - main menu, settings, the gameboard, the gameboard creator, and the pause menu.
        // Each state has its own enter functionality to spawn appropriate entities related to the GUI.
        // Each state has its own exit functionality to despawn entities present before transitioning to the next room.
        app.add_systems(OnEnter(GUIState::MainMenu), setup_main_menu);
        app.add_systems(OnExit(GUIState::MainMenu), cleanup_gui_entities);

        app.add_systems(OnEnter(GUIState::Settings), setup_settings);
        app.add_systems(OnExit(GUIState::Settings), cleanup_gui_entities);

        app.add_systems(OnEnter(GUIState::GameBoardCreator), setup_gameboard_creator);
        app.add_systems(OnExit(GUIState::GameBoardCreator), cleanup_gui_entities);

        app.add_systems(OnEnter(GUIState::GameBoard), setup_gameboard);
        app.add_systems(OnExit(GUIState::GameBoard), cleanup_gui_entities);

        app.add_systems(OnEnter(GUIState::PauseMenu), setup_pause_menu);
        app.add_systems(OnExit(GUIState::PauseMenu), cleanup_gui_entities);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GUIState {
    #[default]
    MainMenu,
    Settings,
    GameBoardCreator,
    GameBoard,
    PauseMenu
}


fn setup_main_menu
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_settings
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_gameboard
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_gameboard_creator
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_pause_menu
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn cleanup_gui_entities
(
    mut commands: Commands,
    entity_query: Query<Entity, With<ActiveEntity>>,
)
{
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
