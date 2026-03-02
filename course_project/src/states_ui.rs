
use bevy::prelude::*;
use crate::spawns::*;

pub struct StatesForUI {}
impl Plugin for StatesForUI {
    fn build(&self, app: &mut App) {

        // Initializing a state type into the application.
        app.init_state::<UIState>();

        // UI State Enter and Exit Definitions
        // These "states" are the UI elements that players engage with - main menu, settings, the gameboard, the gameboard creator, and the pause menu.
        // Each state has its own enter functionality to spawn appropriate entities related to the UI.
        // Each state has its own exit functionality to despawn entities present before transitioning to the next room.
        app.add_systems(OnEnter(UIState::MainMenu), setup_main_menu);
        app.add_systems(OnExit(UIState::MainMenu), cleanup_ui_entities);

        app.add_systems(OnEnter(UIState::Settings), setup_settings);
        app.add_systems(OnExit(UIState::Settings), cleanup_ui_entities);

        app.add_systems(OnEnter(UIState::GameBoardCreator), setup_gameboard_creator);
        app.add_systems(OnExit(UIState::GameBoardCreator), cleanup_ui_entities);

        app.add_systems(OnEnter(UIState::GameBoard), setup_gameboard);
        app.add_systems(OnExit(UIState::GameBoard), cleanup_ui_entities);

        app.add_systems(OnEnter(UIState::PauseMenu), setup_pause_menu);
        app.add_systems(OnExit(UIState::PauseMenu), cleanup_ui_entities);
    }
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


fn setup_main_menu
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
)
    -> Result<()>
{

    let window = window_query.single()?;
    let path_for_image = "sprites/Square.png";
    let path_for_font = "fonts/Cinzel/Cinzel-Bold.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor = 50.0;
    let layer = 1;

    let button_width_of_image = 30.0;
    let button_aspect_ratio = 120.0 / 20.0;
    let button_font_size = 0.02;

    let title_width_of_image = 55.0;
    let title_aspect_ratio = 80.0 / 20.0;
    let title_font_size = 0.06;

    // Title Label
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Label(UILabels::Title),
        path_for_image,
        Vec2::new(x_anchor, 15.0),
        layer,
        title_width_of_image,
        title_aspect_ratio,
        Some(TextSpawn {
            content: "Pentago",
            font_path: path_for_font,
            font_size_scale: title_font_size,
            color: color_of_text,
        })
    );

    // Play Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::Play),
        path_for_image,
        Vec2::new(x_anchor, 45.0),
        layer,
        button_width_of_image,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Play",
            font_path: path_for_font,
            font_size_scale: button_font_size,
            color: color_of_text,
        })
    );

    // Settings Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::Settings),
        path_for_image,
        Vec2::new(x_anchor, 65.0),
        layer,
        button_width_of_image,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Settings",
            font_path: path_for_font,
            font_size_scale: button_font_size,
            color: color_of_text,
        })
    );

    // Exit Game Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::ExitGame),
        path_for_image,
        Vec2::new(x_anchor, 85.0),
        layer,
        button_width_of_image,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Exit Game",
            font_path: path_for_font,
            font_size_scale: button_font_size,
            color: color_of_text,
        })
    );

    Ok(())
}

fn setup_settings
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
)
    -> Result<()>
{
    let window = window_query.single()?;

    Ok(())
}

fn setup_gameboard
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
)
    -> Result<()>
{
    let window = window_query.single()?;


    Ok(())
}

fn setup_gameboard_creator
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
)
    -> Result<()>
{
    let window = window_query.single()?;



    Ok(())
}

fn setup_pause_menu
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
)
    -> Result<()>
{
    let window = window_query.single()?;
    let path_for_image = "sprites/Square.png";
    let path_for_font = "fonts/Cinzel/Cinzel-Bold.ttf";
    let width_of_image = 30.0;
    let aspect_ratio = 100.0 / 20.0;
    let font_size = 0.02;
    let color_of_text = Color::WHITE;
    let layer = 1;

    // Settings Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::Settings),
        path_for_image,
        Vec2::new(50.0, 60.0),
        layer,
        width_of_image,
        aspect_ratio,
        Some(TextSpawn {
            content: "Settings",
            font_path: path_for_font,
            font_size_scale: font_size,
            color: color_of_text,
        })
    );

    Ok(())
}

fn cleanup_ui_entities
(
    mut commands: Commands,
    entity_query: Query<(Entity, &SpawnTypes)>,
)
{
    for (entity, spawn_type) in entity_query.iter() {

        match spawn_type {
            SpawnTypes::UI(_) => { commands.entity(entity).despawn(); }
            _ => {}
        }
    }
}
