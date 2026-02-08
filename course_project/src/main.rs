use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_resource::<LeftClickState>()
        .add_message::<LeftClick>()
        .add_systems(Startup, (spawn_camera, spawn_square).chain())
        .add_systems(Update, (detect_left_click, test_click).chain())
        .add_systems(OnEnter(GameState::Settings), setup_settings)
        .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
        .run();
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Settings,
    GameBoardCreator,
    GameBoard
}

#[derive(Component)]
pub struct ActivelyPlaying {}

#[derive(Component)]
pub struct Square {}

#[derive(Message)]
pub struct LeftClick {}

#[derive(Resource)]
pub struct LeftClickState {
    left_click_occurred: bool
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
    commands.spawn((
        Square {},
        Sprite::from_image(asset_server.load("sprites/Square.png")),
        Transform::from_xyz(window.width() / 1.5, window.height() / 1.5, 0.0),
    ));

    Ok(())
}

fn cleanup_main_menu
(
    mut commands: Commands,
    entity_query: Query<Entity, With<ActivelyPlaying>>,
)
{
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_square
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{

    let window: &Window = window_query.single()?;
    commands.spawn((
        Square {},
        ActivelyPlaying {},
        Sprite::from_image(asset_server.load("sprites/Square.png")),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));

    Ok(())
}

pub fn spawn_camera
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
    ));

    Ok(())
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

impl Default for LeftClickState {
    fn default() -> Self {
        Self {
            left_click_occurred: false
        }
    }
}


// if mouse_event.read().eq(MouseButton::Left) {
//     let cursor_position: Vec2 = window.cursor_position().unwrap();
//     println!("Hello!");
// }
