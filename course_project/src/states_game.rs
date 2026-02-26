
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
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
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    // Creating a window for the game that is set to borderless fullscreen, using the current monitor
    // that the application was opened in, and sets the resolution, refresh rate, and bit depth to
    // what the OS is currently set to.  I've set the window resizable to false because I want to
    // ensure the aspect ratio of the images are maintained (I know there's a way to make the aspect
    // ratio stay the same across resizing, but I'm not implementing that atm - have to look more into it.
    let mut window = windows.single_mut()?;
    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
    window.resizable = false;

    // Creates a camera for the game that is centered on the window's origin point.
    let camera_position: Vec3 = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    spawn_camera(&mut commands, camera_position);

    Ok(())
}

fn setup_settings
(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window = windows.single_mut()?;


    Ok(())
}

fn setup_gameboard
(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window = windows.single_mut()?;


    Ok(())
}

fn setup_gameboard_creator
(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window = windows.single_mut()?;



    Ok(())
}

// Cleanup procedure that occurs for a game state's exit procedure.
fn cleanup_game_entities
(
    mut commands: Commands,
    entity_query: Query<(Entity, &SpawnType)>,
)
{
    for (entity, entity_type) in entity_query.iter()
    {
        if *entity_type == SpawnType::Game  {
            commands.entity(entity).despawn();
        }
    }
}
