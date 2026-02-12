
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::spawns::*;

pub struct StatesForGame {}
impl Plugin for StatesForGame {
    fn build(&self, app: &mut App) {

        // Initializing a state type into the application.
        app.init_state::<GameState>();

        // Game State Enter and Exit Definitions
        // These "states" are the game rooms that players engage with - main menu, settings, the gameboard, and the gameboard creator.
        // Each state has its own enter functionality to spawn appropriate entities related to the room.
        // Each state has its own exit functionality to despawn entities within a room before transitioning to the next room.
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), cleanup_game_entities);
        app.add_systems(OnEnter(GameState::Settings), setup_settings);
        app.add_systems(OnExit(GameState::Settings), cleanup_game_entities);
        app.add_systems(OnEnter(GameState::GameBoard), setup_gameboard);
        app.add_systems(OnExit(GameState::GameBoard), cleanup_game_entities);
        app.add_systems(OnEnter(GameState::GameBoardCreator), setup_gameboard_creator);
        app.add_systems(OnExit(GameState::GameBoardCreator), cleanup_game_entities);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Settings,
    GameBoardCreator,
    GameBoard
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
    let camera_position: Vec3 = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    let square_position: Vec3 = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    spawn_camera(&mut commands, camera_position);
    spawn_square(&mut commands, &asset_server, square_position);

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
    let camera_position: Vec3 = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    let square_position: Vec3 = Vec3::new(window.width() / 1.5, window.height() / 1.5, 0.0);
    spawn_camera(&mut commands, camera_position);
    spawn_square(&mut commands, &asset_server, square_position);

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
    let camera_position: Vec3 = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    let square_position: Vec3 = Vec3::new(window.width() / 1.0, window.height() / 1.0, 0.0);
    spawn_camera(&mut commands, camera_position);
    spawn_square(&mut commands, &asset_server, square_position);

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
    let camera_position: Vec3 = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    let square_position: Vec3 = Vec3::new(window.width() / 0.5, window.height() / 0.5, 0.0);
    spawn_camera(&mut commands, camera_position);
    spawn_square(&mut commands, &asset_server, square_position);

    Ok(())
}

// Cleanup procedure that occurs for a game state's exit procedure.
fn cleanup_game_entities
(
    mut commands: Commands,
    entity_query: Query<Entity, With<ActiveEntity>>,
)
{
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
