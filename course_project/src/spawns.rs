use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::resources::{ButtonChain, UIStateHistory};
use crate::states_ui::UIState;

pub struct Spawns {}
impl Plugin for Spawns {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (resize_text_spawn, handle_ui_button_interactions).chain());
    }
}

#[derive(Component, PartialEq)]
pub enum UIContainers {
    PauseMenu,
    Confirmation,
}

#[derive(Component, PartialEq, Clone)]
pub enum UIButtons {
    Play,
    Settings,
    ExitGame,
    MainMenu,
    Back,
    Resume,
    CreateBoard,
    PauseMenu,
    Yes,
    No,
}

#[derive(Component, PartialEq)]
pub enum UILabels {
    Title,
    Pause,
    Confirmation,
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



pub fn spawn_camera
(
    commands: &mut Commands,
    position: Vec3,
)
{
    commands.spawn((
        Camera2d,
        CameraSpawns::CenteredOnWindow,
        Transform::from_translation(position)
    ));
}



pub fn spawn_ui_element
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    ui_button: Option<UIButtons>,
    ui_container: Option<UIContainers>,
    ui_label: Option<UILabels>,
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

    // Calculating UI component size (relative to width of window).
    let width_half_size = size_of_element / 2.0;
    let height_half_size = width_half_size / aspect_ratio;

    // Assigning UI attributes - image, position, layer, and size.
    let mut entity = commands.spawn((
        Button,
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
    ));

    // Declaring Types for Entity (If Any Were Provided)
    if let Some(button) = ui_button {
        entity.insert(button);
    }
    if let Some(container) = ui_container {
        entity.insert(container);
    }
    if let Some(label) = ui_label {
        entity.insert(label);
    }

    // Assigning text to the UI element if any text was provided.
    entity.with_children(|parent| {
        if let Some(text_spawn) = text {
            parent.spawn((
                Text::new(text_spawn.content),
                TextColor(text_spawn.color),
                TextLayout::new_with_justify(Justify::Center),
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

pub fn spawn_confirmation
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    dialog_text: &'static str,
)
    -> Result<()>
{
    // Container
    spawn_ui_element(
        commands, asset_server, window,
        None,
        Some(UIContainers::Confirmation),
        None,
        "sprites/DarkSquare.png",
        Vec3::new(50.0, 40.0, 3.0),
        30.0,
        100.0 / 50.0,
        None
    );

    // Label
    spawn_ui_element(
        commands, asset_server, window,
        None,
        None,
        Some(UILabels::Confirmation),
        "sprites/Square.png",
        Vec3::new(50.0, 40.0, 4.0),
        20.0,
        100.0 / 20.0,
        Some(TextSpawn {
            content: dialog_text,
            font_path: "fonts/Spectral/Spectral-Medium.ttf",
            font_size_scale: 0.015,
            color: Color::WHITE,
        })
    );

    // Yes Button
    spawn_ui_element(
        commands, asset_server, window,
        Some(UIButtons::Yes),
        None,
        None,
        "sprites/Square.png",
        Vec3::new(45.0, 50.0, 4.0),
        5.0,
        100.0 / 50.0,
        Some(TextSpawn {
            content: "YES",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // No Button
    spawn_ui_element(
        commands, asset_server, window,
        Some(UIButtons::No),
        None,
        None,
        "sprites/Square.png",
        Vec3::new(55.0, 50.0, 4.0),
        5.0,
        100.0 / 50.0,
        Some(TextSpawn {
            content: "NO",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

pub fn despawn_confirmation
(
    commands: &mut Commands,
    button_query: &Query<(Entity, &UIButtons)>,
    container_query: &Query<(Entity, &UIContainers)>,
    label_query: &Query<(Entity, &UILabels)>,
)
{
    // Delete Confirmation Buttons
    for (entity, button) in button_query.iter() {
        if *button == UIButtons::Yes || *button == UIButtons::No {
            commands.entity(entity).despawn();
        }
    }

    // Delete Container for Confirmation Dialogs
    for (entity, container) in container_query.iter() {
        if *container == UIContainers::Confirmation {
            commands.entity(entity).despawn();
        }
    }

    // Delete Labels within Confirmation Dialogs
    for (entity, label) in label_query.iter() {
        if *label == UILabels::Confirmation {
            commands.entity(entity).despawn();
        }
    }
}

// RESIZE_TEXT_SPAWN
// This will resize text to always be relative to a text's set scaling factor and the
// window's current width.  I say "current" since this system is running every frame but its
// code will only trigger when the window gets resized.
pub fn resize_text_spawn
(
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
// Buttons are programmed out based on enum type and will direct state transitions and
// trigger confirmation dialogs where appropriate.
pub fn handle_ui_button_interactions
(
    interaction_query: Query<(&Interaction, &UIButtons), Changed<Interaction>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    mut button_chain: ResMut<ButtonChain>,
    mut next_state: ResMut<NextState<UIState>>,
    mut state_history: ResMut<UIStateHistory>,
    mut app_exit: MessageWriter<AppExit>,
    button_query: Query<(Entity, &UIButtons)>,
    container_query: Query<(Entity, &UIContainers)>,
    label_query: Query<(Entity, &UILabels)>,
)
    -> Result<()>
{
    for (interaction, button) in interaction_query.iter() {

        if *interaction == Interaction::Pressed {

            match (button_chain.as_slice(), button) {

                ([UIButtons::MainMenu], UIButtons::Yes) => {
                    button_chain.clear();
                    despawn_confirmation(&mut commands, &button_query, &container_query, &label_query);
                    state_history.clear();
                    next_state.set(UIState::MainMenu);
                },

                ([UIButtons::ExitGame], UIButtons::Yes) => {
                    button_chain.clear();
                    despawn_confirmation(&mut commands, &button_query, &container_query, &label_query);
                    app_exit.write(AppExit::Success);
                },

                ([], UIButtons::MainMenu) => {
                    let window = window_query.single()?;
                    spawn_confirmation(&mut commands, &asset_server, &window, "End the board and navigate to the Main Menu?")?;
                    button_chain.push(UIButtons::MainMenu);
                },

                ([], UIButtons::ExitGame) => {
                    let window = window_query.single()?;
                    spawn_confirmation(&mut commands, &asset_server, &window, "Close the program and exit the game?")?;
                    button_chain.push(UIButtons::ExitGame);
                },

                (_, UIButtons::No) => {
                    button_chain.clear();
                    despawn_confirmation(&mut commands, &button_query, &container_query, &label_query);
                },

                (_, UIButtons::Back) => {
                    button_chain.clear();
                    let previous_state = state_history.pop().unwrap_or(UIState::MainMenu);
                    next_state.set(previous_state);
                },

                (_, UIButtons::Play)        => { button_chain.clear(); next_state.set(UIState::GameBoardCreator); },
                (_, UIButtons::Settings)    => { button_chain.clear(); next_state.set(UIState::Settings); },
                (_, UIButtons::Resume)      => { button_chain.clear(); next_state.set(UIState::GameBoard); },
                (_, UIButtons::CreateBoard) => { button_chain.clear(); next_state.set(UIState::GameBoard); },
                (_, UIButtons::PauseMenu)   => { button_chain.clear(); next_state.set(UIState::PauseMenu); },

                _ => { button_chain.clear(); }
            }
        }
    }

    Ok(())
}
