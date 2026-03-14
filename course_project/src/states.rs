
use bevy::prelude::*;
use crate::resources::StateHistory;
use crate::spawns::*;

pub struct States {}
impl Plugin for States {
    fn build(&self, app: &mut App) {
        app.init_state::<State>();

        app.add_systems(OnEnter(State::MainMenu), setup_main_menu);
        app.add_systems(OnExit(State::MainMenu), (record_main_menu_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(State::Settings), setup_settings);
        app.add_systems(OnExit(State::Settings), (record_settings_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(State::GameBoardCreator), setup_gameboard_creator);
        app.add_systems(OnExit(State::GameBoardCreator), (record_gameboard_creator_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(State::GameBoard), setup_gameboard);
        app.add_systems(OnExit(State::GameBoard), (record_gameboard_exit, cleanup_ui_entities).chain());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum State {
    #[default]
    MainMenu,
    Settings,
    GameBoardCreator,
    GameBoard,
}

// UI SETUPS
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {


    // Defining variables for UI elements.
    let window = window_query.single()?;
    let path_for_image: Option<&'static str> = Some("sprites/Square.png");
    let path_for_font = "fonts/Cinzel/Cinzel-Bold.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor = 50.0;
    let layer = 1.0;

    let button_width = 30.0;
    let button_aspect_ratio: Option<f32> = Some(120.0 / 20.0);
    let button_font_size = 0.02;

    let title_width = 55.0;
    let title_aspect_ratio: Option<f32> = Some(80.0 / 20.0);
    let title_font_size = 0.06;

    // Title Label
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::MainMenu),
        None,
        Some(Labels::Title),
        None,
        Vec3::new(x_anchor, 25.0, layer),
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
        &mut commands, &asset_server, window,
        Some(Buttons::Play),
        None,
        None,
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
        &mut commands, &asset_server, window,
        Some(Buttons::Settings),
        None,
        None,
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
        &mut commands, &asset_server, window,
        Some(Buttons::ExitGame),
        None,
        None,
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
        &mut commands, &asset_server, window,
        Some(Buttons::Back),
        None,
        None,
        Some("sprites/Square.png"),
        Vec3::new(10.0, 5.0, 1.0),
        15.0,
        Some(8.0 / 2.0),
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
        &mut commands, &asset_server, window,
        Some(Buttons::Pause),
        None,
        None,
        Some("sprites/Square.png"),
        Vec3::new(10.0, 5.0, 1.0),
        15.0,
        Some(8.0 / 2.0),
        Some(TextSpawn {
            content: "Pause",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    Ok(())
}
fn setup_gameboard_creator(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) -> Result<()> {

    let window = window_query.single()?;

    // Back Button
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::Back),
        None,
        None,
        Some("sprites/Square.png"),
        Vec3::new(10.0, 5.0, 1.0),
        15.0,
        Some(8.0 / 2.0),
        Some(TextSpawn {
            content: "Back",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    // Create Board Button
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::CreateBoard),
        None,
        None,
        Some("sprites/Square.png"),
        Vec3::new(90.0, 90.0, 1.0),
        15.0,
        Some(8.0 / 2.0),
        Some(TextSpawn {
            content: "Create Board",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::MainMenu),
        None,
        None,
        Some("sprites/Untitled.png"),
        Vec3::new(50.0, 50.0, 1.0),
        15.0,
        Some(0.33),
        None,
    );

    Ok(())
}

// UI STATE RECORDERS
fn record_main_menu_exit(mut history: ResMut<StateHistory>) { history.push(State::MainMenu); }
fn record_settings_exit(mut history: ResMut<StateHistory>) { history.push(State::Settings); }
fn record_gameboard_exit(mut history: ResMut<StateHistory>) { history.push(State::GameBoard); }
fn record_gameboard_creator_exit(mut history: ResMut<StateHistory>) { history.push(State::GameBoardCreator); }

// TRASH COLLECTOR
fn cleanup_ui_entities
(
    mut commands: Commands,
    button_query: Query<Entity, With<Buttons>>,
    container_query: Query<Entity, With<Containers>>,
    label_query: Query<Entity, With<Labels>>,
)
{
    for entity in button_query.iter()       { commands.entity(entity).despawn(); }
    for entity in container_query.iter()    { commands.entity(entity).despawn(); }
    for entity in label_query.iter()        { commands.entity(entity).despawn(); }
}
