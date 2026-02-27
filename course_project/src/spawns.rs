
use bevy::prelude::*;
use bevy::window::WindowResized;

pub struct Spawns {}
impl Plugin for Spawns {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, (resize_text_spawn).chain());

    }
}

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

// This component is always built into other elements - or at least it should be, using it
// on its own will make cleanup features not work appropriately.  Text made from this element
// will be deleted when its corresponding parent is deleted.
#[derive(Component)]
pub struct TextSpawn {
    content: &'static str,      // content is always known at compile time, hence static lifetime.
    font_path: &'static str,    // font_path is always known at compile time, hence static lifetime.
    font_size_scale: f32,       // font_size_scale uses the window width as it's factor (use values below 1.0).
    color: Color,
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
    path_for_image: &'static str,   // PATH_FOR_IMAGE : This takes in the file path for the image you're trying to use for the GUI element.
    position: Vec2,                 // POSITION : Percentage based with origin centered at the top left of the window.
    layer: i32,                     // LAYER : Designates the depth of where the GUI element will be on the UI.
    size_of_element: f32,           // SIZE_OF_ELEMENT : Size is based on the width of the window and is percentage based.
                                    //      A value of 20.0 equals 20% of the window's width.  You use this value to
                                    //      determine the overall image size of the GUI element.
    image_width: f32,               // IMAGE_WIDTH : Ensure that the image_width matches the pixel width of the image.
    image_height: f32,              // IMAGE_HEIGHT : Ensure that the image_height matches the pixel height of the image.
    text: Option<TextSpawn>,          // TEXT : This is an optional element, by using it text can be placed onto a GUI element.
                                    //      Position of the text is relative to the image that the GUI element uses.  You can
                                    //      pass None into a call of this function if an element isn't supposed to contain text.
)
{
    // We calculate the aspect_ratio to ensure that the image which is produced for a button
    // is always relative to the image's width and height.
    let calculated_aspect_ratio = image_width / image_height;

    // We calculate the half sizes so that we can create an offset in the position of the image that
    // spawns to ensure that elements are positioned based on their image's center and not the top
    // left corner of an image.
    // This offset is relative to the window size since the size of the element is relative to the window.
    let width_half_size = size_of_element / 2.0;
    let height_half_size = width_half_size / calculated_aspect_ratio;

    commands.spawn((
        Button,
        gui_spawn_type,
        SpawnType::Active,
        ZIndex(layer),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(position.x - width_half_size),
            top: Val::Percent(position.y - height_half_size),
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
    )).with_children(|parent| {

        // If a TextSpawn argument exists, make one.  Otherwise, ignore this code.
        if let Some(text_spawn) = text {

            parent.spawn((

                Text::new(text_spawn.content),

                TextFont {
                    font: asset_server.load(text_spawn.font_path),
                    font_size: text_spawn.font_size_scale,
                    ..default()
                },

                TextColor(text_spawn.color),
                text_spawn
            ));
        }
    });
}



// BUTTONS FOR GAME
pub fn spawn_button_play ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonPlay,
        "sprites/Square.png",
        Vec2::new(50.0, 40.0),
        1,
        30.0,
        512.0,
        96.0,
        Some(TextSpawn {
            content: "Play",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.02,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_settings ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonSettings,
        "sprites/Square.png",
        Vec2::new(50.0, 60.0),
        1,
        30.0,
        512.0,
        96.0,
        Some(TextSpawn {
            content: "Settings",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.02,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_exit_game ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonExitGame,
        "sprites/Square.png",
        Vec2::new(50.0, 80.0),
        1,
        30.0,
        512.0,
        96.0,
        Some(TextSpawn {
            content: "Exit Game",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.02,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_main_menu ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonMainMenu,
        "sprites/Square.png",
        Vec2::new(10.0, 10.0),
        1,
        30.0,
        512.0,
        256.0,
        Some(TextSpawn {
            content: "Main Menu",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.03,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_back ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonBack,
        "sprites/Square.png",
        Vec2::new(10.0, 10.0),
        1,
        30.0,
        512.0,
        256.0,
        Some(TextSpawn {
            content: "Back",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.03,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_resume ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonResume,
        "sprites/Square.png",
        Vec2::new(10.0, 10.0),
        1,
        30.0,
        512.0,
        256.0,
        Some(TextSpawn {
            content: "Resume",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.03,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_create_board ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonCreateBoard,
        "sprites/Square.png",
        Vec2::new(10.0, 10.0),
        1,
        30.0,
        512.0,
        256.0,
        Some(TextSpawn {
            content: "Create Board",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.03,
            color: Color::WHITE,
        })
    );
}
pub fn spawn_button_pause_menu ( commands: &mut Commands, asset_server: &AssetServer ) {

    spawn_gui_element(
        commands,
        asset_server,
        GUISpawns::ButtonPauseMenu,
        "sprites/Square.png",
        Vec2::new(10.0, 10.0),
        1,
        30.0,
        512.0,
        256.0,
        Some(TextSpawn {
            content: "Pause",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.03,
            color: Color::WHITE,
        })
    );
}



// RESIZE_TEXT_SPAWN
// This will resize text to always be relative to a text's set scaling factor and the
// window's current width.  I say "current" since this system is running every Update but its
// code will only trigger when the window gets resized.
pub fn resize_text_spawn(
    mut text_query: Query<(&mut TextFont, &TextSpawn)>,
    window_query: Query<&Window>,
    mut resize_reader: MessageReader<WindowResized>,
)
    -> Result<()>
{
    for _ in resize_reader.read() {

        let window = window_query.single()?;

        for (mut text_font, text_spawn) in text_query.iter_mut() {
            text_font.font_size = window.width() * text_spawn.font_size_scale;
        }
    }

    Ok(())
}
