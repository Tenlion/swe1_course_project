
use bevy::prelude::*;
use crate::resources::UIStateHistory;
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
        app.add_systems(OnExit(UIState::MainMenu), (record_main_menu_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(UIState::Settings), setup_settings);
        app.add_systems(OnExit(UIState::Settings), (record_settings_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(UIState::GameBoardCreator), setup_gameboard_creator);
        app.add_systems(OnExit(UIState::GameBoardCreator), (record_gameboard_creator_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(UIState::GameBoard), setup_gameboard);
        app.add_systems(OnExit(UIState::GameBoard), (record_gameboard_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(UIState::PauseMenu), setup_pause_menu);
        app.add_systems(OnExit(UIState::PauseMenu), (record_pause_menu_exit, cleanup_ui_entities).chain());
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

// UI SETUPS
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {

    // Universal Main Menu Variables
    let window = window_query.single()?;
    let path_for_image = "sprites/Square.png";
    let path_for_font = "fonts/Cinzel/Cinzel-Bold.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor = 50.0;
    let layer = 1.0;

    // Button Variables
    let button_width = 30.0;
    let button_aspect_ratio = 120.0 / 20.0;
    let button_font_size = 0.02;

    // Title Variables
    let title_width = 55.0;
    let title_aspect_ratio = 80.0 / 20.0;
    let title_font_size = 0.06;

    // Title Label
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Label(UILabels::Title),
        path_for_image,
        Vec3::new(x_anchor, 15.0, layer),
        title_width,
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
        Vec3::new(x_anchor, 45.0, layer),
        button_width,
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
        Vec3::new(x_anchor, 65.0, layer),
        button_width,
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
        Vec3::new(x_anchor, 85.0, layer),
        button_width,
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
fn setup_settings(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {

    let window = window_query.single()?;

    // Back Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::Back),
        "sprites/Square.png",
        Vec3::new(10.0, 5.0, 1.0),
        15.0,
        8.0 / 2.0,
        Some(TextSpawn {
            content: "Back",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    Ok(())
}
fn setup_gameboard(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {

    let window = window_query.single()?;

    // Pause Menu Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::PauseMenu),
        "sprites/Square.png",
        Vec3::new(10.0, 5.0, 1.0),
        15.0,
        8.0 / 2.0,
        Some(TextSpawn {
            content: "Pause",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    Ok(())
}
fn setup_gameboard_creator (mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {

    let window = window_query.single()?;

    // Back Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::Back),
        "sprites/Square.png",
        Vec3::new(10.0, 5.0, 1.0),
        15.0,
        8.0 / 2.0,
        Some(TextSpawn {
            content: "Back",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    // Create Board Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::CreateBoard),
        "sprites/Square.png",
        Vec3::new(90.0, 90.0, 1.0),
        15.0,
        8.0 / 2.0,
        Some(TextSpawn {
            content: "Create Board",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    Ok(())
}
fn setup_pause_menu (mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {

    // Universal Pause UI Variables
    let window = window_query.single()?;
    let path_for_image = "sprites/Square.png";
    let path_for_font = "fonts/Cinzel/Cinzel-Bold.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor: f32 = 50.0;
    let layer: f32 = 2.0;

    // Container Variables
    let image_for_container = "sprites/DarkSquare.png";
    let container_layer: f32 = 1.0;

    // Button Variables
    let button_width = 10.0;
    let button_aspect_ratio = 100.0 / 20.0;
    let button_font_size = 0.01;

    // Label Variables
    let pause_label_width = 15.0;
    let pause_label_aspect_ratio = 80.0 / 20.0;
    let pause_label_font_size = 0.015;

    // Pause Menu Container
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Container(UIContainers::PauseMenu),
        image_for_container,
        Vec3::new(x_anchor, 40.0, container_layer),
        20.0,
        20.0 / 28.0,
        None,
    );

    // Pause Menu Label
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Label(UILabels::Pause),
        path_for_image,
        Vec3::new(x_anchor, 35.0, layer),
        pause_label_width,
        pause_label_aspect_ratio,
        Some(TextSpawn {
            content: "Pause Menu",
            font_path: path_for_font,
            font_size_scale: pause_label_font_size,
            color: color_of_text,
        })
    );

    // Resume Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::Resume),
        path_for_image,
        Vec3::new(x_anchor, 45.0, layer),
        button_width,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Resume",
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
        Vec3::new(x_anchor, 52.5, layer),
        button_width,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Settings",
            font_path: path_for_font,
            font_size_scale: button_font_size,
            color: color_of_text,
        })
    );

    // Main Menu Button
    spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        UISpawnTypes::Button(UIButtons::MainMenu),
        path_for_image,
        Vec3::new(x_anchor, 60.0, layer),
        button_width,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Main Menu",
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
        Vec3::new(x_anchor, 67.5, layer),
        button_width,
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

// UI STATE RECORDERS
// Functions that add to the UI state history when a scene a UI state has been changed.
fn record_main_menu_exit(mut history: ResMut<UIStateHistory>) {
    history.push(UIState::MainMenu);
}
fn record_settings_exit(mut history: ResMut<UIStateHistory>) {
    history.push(UIState::Settings);
}
fn record_gameboard_creator_exit(mut history: ResMut<UIStateHistory>) {
    history.push(UIState::GameBoardCreator);
}
fn record_gameboard_exit(mut history: ResMut<UIStateHistory>) {
    history.push(UIState::GameBoard);
}
fn record_pause_menu_exit(mut history: ResMut<UIStateHistory>) {
    history.push(UIState::PauseMenu);
}

// TRASH COLLECTOR
fn cleanup_ui_entities(mut commands: Commands, entity_query: Query<(Entity, &SpawnTypes)>) {
    for (entity, spawn_type) in entity_query.iter() {

        match spawn_type {
            SpawnTypes::UI(_) => { commands.entity(entity).despawn(); }
            _ => {}
        }
    }
}
