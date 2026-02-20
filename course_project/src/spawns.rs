
use bevy::prelude::*;

#[derive(Component)]
pub struct Square {}

#[derive(Component)]
pub struct ActiveEntity {}

#[derive(Component)]
pub struct GameEntity {}

#[derive(Component)]
pub struct GUIEntity {}

#[derive(Component)]
pub struct CameraEntity {}

pub fn spawn_square
(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3
)
{
    commands.spawn((
        Square {},
        ActiveEntity {},
        GameEntity {},
        Sprite::from_image(asset_server.load("sprites/Square.png")),
        Transform::from_translation(position),
    ));
}

pub fn spawn_camera
(
    commands: &mut Commands,
    position: Vec3,
)
{
    commands.spawn((
        Camera2d,
        ActiveEntity {},
        CameraEntity {},
        Transform::from_translation(position)
    ));
}
