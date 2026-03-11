
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::resources::UIStateHistory;
use crate::states_ui::UIState;

pub struct Spawns {}
impl Plugin for Spawns {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, (resize_text_spawn, handle_ui_button_interactions).chain());

    }
}

#[derive(Component, PartialEq)]
pub enum SpawnTypes {
    Game(GameSpawnTypes),
    UI(UISpawnTypes),
    Camera(CameraSpawns),
}

#[derive(Component, PartialEq)]
pub enum UISpawnTypes {
    Container(UIContainers),
    Button(UIButtons),
    Label(UILabels),
}

#[derive(Component, PartialEq)]
pub enum UIContainers {
    PauseMenu,
}

#[derive(Component, PartialEq)]
pub enum UIButtons {
    Play,
    Settings,
    ExitGame,
    MainMenu,
    Back,
    Resume,
    CreateBoard,
    PauseMenu,
}

#[derive(Component, PartialEq)]
pub enum UILabels {
    Title,
    Pause,
}

#[derive(Component, PartialEq)]
pub enum GameSpawnTypes {
    Board,
    Subboard,
    Node,
}

#[derive(Component, PartialEq)]
pub enum CameraSpawns {
    CenteredOnWindow,
}

// This component is always built into other elements - or at least it should be, using it
// on its own will make cleanup features not work appropriately.  Text made from this element
// will be deleted when its corresponding parent is deleted.
#[derive(Component)]
pub struct TextSpawn {
    pub content: &'static str,      // content is always known at compile time, hence static lifetime.
    pub font_path: &'static str,    // font_path is always known at compile time, hence static lifetime.
    pub font_size_scale: f32,       // font_size_scale uses the window width as it's factor (use values below 1.0).
    pub color: Color,
}



pub fn spawn_camera(
    commands: &mut Commands,
    position: Vec3,
) {
    commands.spawn((
        Camera2d,
        SpawnTypes::Camera(CameraSpawns::CenteredOnWindow),
        Transform::from_translation(position)
    ));
}



pub fn spawn_ui_element
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    ui_spawn_type: UISpawnTypes,
    path_for_image: &'static str,   // PATH_FOR_IMAGE : This takes in the file path for the image you're trying to use for the UI element.
    position: Vec3,                 // POSITION : Percentage based with origin centered at the top left of the window.  Z values should be discrete.
    size_of_element: f32,           // SIZE_OF_ELEMENT : Size is based on the width of the window and is percentage based.
                                    //      A value of 20.0 equals 20% of the window's width.  You use this value to
                                    //      determine the overall image size of the UI element.
    aspect_ratio: f32,              // ASPECT_RATIO : Can manipulate the size of an element.  Best to throw in calculated values
                                    //      16 (width) / 9 (height) so that one can understand the difference between the width and height.
    text: Option<TextSpawn>,        // TEXT : This is an optional element, by using it text can be placed onto a UI element.
                                    //      Position of the text is relative to the image that the UI element uses.  You can
                                    //      pass None into a call of this function if an element isn't supposed to contain text.
)
{

    // We calculate the half sizes so that we can create an offset in the position of the image that
    // spawns to ensure that elements are positioned based on their image's center and not the top
    // left corner of an image.
    // This offset is relative to the window size since the size of the element is relative to the window.
    let width_half_size = size_of_element / 2.0;
    let height_half_size = width_half_size / aspect_ratio;

    commands.spawn((
        Button,
        SpawnTypes::UI(ui_spawn_type),
        ZIndex(position.z as i32),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(position.x - width_half_size),
            top: Val::Percent(position.y - height_half_size),
            width: Val::Percent(size_of_element),
            height: Val::Auto,
            aspect_ratio: Some(aspect_ratio),
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
                TextColor(text_spawn.color),
                TextFont {
                    font: asset_server.load(text_spawn.font_path),
                    font_size: window.width() * text_spawn.font_size_scale,
                    ..default()
                },
                text_spawn,
            ));
        }
    });
}



// RESIZE_TEXT_SPAWN
// This will resize text to always be relative to a text's set scaling factor and the
// window's current width.  I say "current" since this system is running every frame but its
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



// INTERACTION EVENTS FOR UI BUTTONS
// Buttons are programmed out based on enum type and will direct
pub fn handle_ui_button_interactions(
    interaction_query: Query<(&Interaction, &SpawnTypes), Changed<Interaction>>,
    mut next_state: ResMut<NextState<UIState>>,
    mut state_history: ResMut<UIStateHistory>,
    mut app_exit: MessageWriter<AppExit>,
)
{
    for (interaction, spawn_type) in interaction_query.iter() {

        if *interaction == Interaction::Pressed {

            if let SpawnTypes::UI(UISpawnTypes::Button(button)) = spawn_type {

                match button {
                    UIButtons::Play         => next_state.set(UIState::GameBoardCreator),
                    UIButtons::Settings     => next_state.set(UIState::Settings),
                    UIButtons::Resume       => next_state.set(UIState::GameBoard),
                    UIButtons::CreateBoard  => next_state.set(UIState::GameBoard),
                    UIButtons::PauseMenu    => next_state.set(UIState::PauseMenu),

                    // These are set up like this temporarily.  Later on I would like to add a
                    // confirmation dialog before navigating to the main menu and before closing the game.
                    UIButtons::ExitGame => { app_exit.write(AppExit::Success); },
                    UIButtons::MainMenu => {
                        state_history.clear();
                        next_state.set(UIState::MainMenu);
                    },

                    UIButtons::Back => {
                        // If the state_history stack is empty, this will cause a user to go back to the MainMenu
                        // instead of having the program crash due to an empty stack.
                        let previous_state = state_history.pop().unwrap_or(UIState::MainMenu);
                        next_state.set(previous_state);
                    },
                }
            }
        }
    }
}
