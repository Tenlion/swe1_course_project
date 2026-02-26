
use bevy::prelude::*;

// Spawn Types
#[derive(Component, PartialEq)]
pub enum SpawnType
{
    Active,
    Inactive,
    Game,
    GUI,
    Camera,
}

// GUI Spawn Types
#[derive(Component, PartialEq)]
pub enum GUISpawns
{
    ButtonPlay,
    ButtonSettings,
    ButtonExitGame,
    ButtonMainMenu,
    ButtonBack,
    ButtonResume,
    ButtonCreateBoard,
    ButtonPauseMenu,
}

// Game Spawn Types
#[derive(Component, PartialEq)]
pub enum GameSpawns
{
    Board,
    Subboard,
    Node
}

// Camera Spawn Types
#[derive(Component, PartialEq)]
pub enum CameraSpawns
{
    CenteredOnWindow,
}



pub fn spawn_camera
(
    commands: &mut Commands,
    position: Vec3,
)
{
    commands.spawn((
        Camera2d,
        SpawnType::Active,
        CameraSpawns::CenteredOnWindow,
        Transform::from_translation(position)
    ));
}



pub fn spawn_gui_element
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    gui_spawn_type: GUISpawns,
    path_for_image: String,         // PATH_FOR_IMAGE : This takes in the file path for the image you're trying to use for the GUI element.
    position: Vec2,                 // POSITION : Percentage based with origin centered at the top left of the window.
    layer: i32,                     // LAYER : Designates the depth of where the GUI element will be on the UI.
    size_of_element: f32,           // SIZE_OF_ELEMENT : Size is based on the width of the window and is percentage based.
                                    //      A value of 20.0 equals 20% of the window's width.  You use this value to
                                    //      determine the overall image size of the GUI element.
    image_width: f32,               // IMAGE_WIDTH : Ensure that the image_width matches the pixel width of the image.
    image_height: f32               // IMAGE_HEIGHT : Ensure that the image_height matches the pixel height of the image.
)
{
    // We calculate the aspect_ratio to ensure that the image which is produced for a button
    // is always relative to the image's width and height.
    let calculated_aspect_ratio = image_width / image_height;

    commands.spawn((
        Button,
        gui_spawn_type,
        SpawnType::Active,
        ZIndex(layer),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(position.x),
            top: Val::Percent(position.y),
            width: Val::Percent(size_of_element),
            height: Val::Auto,
            aspect_ratio: Some(calculated_aspect_ratio),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode {
            image: asset_server.load(path_for_image),
            ..default()
        },
    ));
}

pub fn spawn_button_play
(
    commands: &mut Commands,
    asset_server: &AssetServer,
)
{
    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonPlay,
        "sprites/Square.png".to_string(),
        Vec2::new(10.0, 10.0),
        1,
        10.0,
        256.0,
        256.0
    );
}
