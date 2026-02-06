use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        //.add_message::<LeftClick>()
        .add_systems(Startup, (spawn_node, spawn_camera).chain())
        .add_systems(Update, (change_node).chain())
        .run();
}

#[derive(Component)]
pub struct Node {}

#[derive(Component)]
pub struct Board {}

#[derive(Component)]
pub struct SubBoard {}

// #[derive(Message)]
// pub struct LeftClick {}

pub fn spawn_node
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;
    commands.spawn((
        Node {},
        Sprite::from_image(asset_server.load("sprites/Square.png")),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));

    Ok(())
}

pub fn spawn_camera
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
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

// pub fn detect_left_click (
//     mouse_input: Res<ButtonInput<MouseButton>>,
//     mut left_click_event: MessageWriter<LeftClick>
// )
// {
//     if mouse_input.just_pressed(MouseButton::Left) {
//         left_click_event.write(LeftClick {});
//     }
// }

pub fn change_node (
    mut mouse_event: MessageReader<MouseButtonInput>,
    mut node_query: Query<(&mut Transform, &Node)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;
    for mouse_button_input in mouse_event.read() {
        info!("{:?}", mouse_button_input);
    }
    for (mut transform, node) in node_query.iter_mut() {
        for mouse_button_input in mouse_event.read() {
            info!("{:?}", mouse_button_input);
        }

        // if mouse_event.read().eq(MouseButton::Left) {
        //     let cursor_position: Vec2 = window.cursor_position().unwrap();
        //     println!("Hello!");
        // }
    }

    Ok(())
}
