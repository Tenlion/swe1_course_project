
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::resources::{ButtonChain, StateHistory};
use crate::states::State;

pub struct Spawns {}
impl Plugin for Spawns {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (resize_text_spawn, handle_ui_button_interactions).chain());
    }
}

#[derive(Component, PartialEq)]
pub enum Containers {
    Pause,
    Confirmation,
}

#[derive(Component, PartialEq, Clone)]
pub enum Buttons {
    Play,
    Settings,
    ExitGame,
    MainMenu,
    Back,
    Resume,
    CreateBoard,
    Pause,
    Yes,
    No,
}

#[derive(Component, PartialEq)]
pub enum Labels {
    Title,
    Pause,
    Confirmation,
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




pub fn spawn_ui_element
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    ui_button: Option<Buttons>,
    ui_container: Option<Containers>,
    ui_label: Option<Labels>,
    path_for_image: Option<&'static str>,   // PATH_FOR_IMAGE : This takes in the file path for the image you're trying to use for the UI element.
    position: Vec3,                         // POSITION : Percentage based with origin centered at the top left of the window.  Z values should be discrete.
    size_of_element: f32,                   // SIZE_OF_ELEMENT : Size is based on the width of the window and is percentage based.
                                            //      A value of 20.0 equals 20% of the window's width.  You use this value to
                                            //      determine the overall image size of the UI element.
    aspect_ratio: Option<f32>,              // ASPECT_RATIO : Can manipulate the ratio dimensions of an element.  Best to throw in calculated values
                                            //      16 (width) / 9 (height) so that one can understand the difference between the width and height.
    text: Option<TextSpawn>,                // TEXT : This is an optional element, by using it text can be placed onto a UI element.
                                            //      Position of the text is relative to the image that the UI element uses.  You can
                                            //      pass None into a call of this function if an element isn't supposed to contain text.
)
    -> Entity
{

    // Calculating UI component size (relative to width of window).
    let width_half_size = size_of_element / 2.0;
    let height_half_size = width_half_size / aspect_ratio.unwrap_or(1.0);

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
            aspect_ratio,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));

    // Applying an image to the UI element if one was passed in.
    if let Some(image_path) = path_for_image {
        entity.insert(ImageNode {
            image: asset_server.load(image_path),
            ..default()
        });
    }

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

    entity.id()
}

// SPAWN CONFIRMATION
// Used to create confirmation dialogs that can have different text within them based on what's
// passed into dialog_text.
pub fn spawn_confirmation
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    dialog_text: &'static str,
)
{
    // Container
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Confirmation),
        None,
        Some("sprites/DarkSquare.png"),
        Vec3::new(50.0, 40.0, 3.0),
        35.0,
        Some(100.0 / 50.0),
        None
    );

    // Label
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Confirmation),
        Some(Labels::Confirmation),
        None,
        Vec3::new(50.0, 40.0, 4.0),
        28.0,
        Some(100.0 / 20.0),
        Some(TextSpawn {
            content: dialog_text,
            font_path: "fonts/Spectral/Spectral-Medium.ttf",
            font_size_scale: 0.013,
            color: Color::WHITE,
        })
    );

    // Yes Button
    spawn_ui_element(
        commands,
        asset_server,
        window,
        Some(Buttons::Yes),
        Some(Containers::Confirmation),
        None,
        Some("sprites/Square.png"),
        Vec3::new(45.0, 50.0, 4.0),
        5.0,
        Some(100.0 / 50.0),
        Some(TextSpawn {
            content: "YES",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // No Button
    spawn_ui_element(
        commands,
        asset_server,
        window,
        Some(Buttons::No),
        Some(Containers::Confirmation),
        None,
        Some("sprites/Square.png"),
        Vec3::new(55.0, 50.0, 4.0),
        5.0,
        Some(100.0 / 50.0),
        Some(TextSpawn {
            content: "NO",
            font_path: "fonts/Cinzel/Cinzel-Bold.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );
}

// SPAWN PAUSE
//
pub fn spawn_pause
(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
)
{
    let path_for_image: Option<&'static str> = Some("sprites/Square.png");
    let path_for_font = "fonts/Cinzel/Cinzel-Bold.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor: f32 = 50.0;
    let layer: f32 = 2.0;

    let image_for_container: Option<&'static str> = Some("sprites/DarkSquare.png");
    let container_layer: f32 = 1.0;

    let button_width = 10.0;
    let button_aspect_ratio: Option<f32> = Some(100.0 / 20.0);
    let button_font_size = 0.01;

    let pause_label_width = 15.0;
    let pause_label_aspect_ratio: Option<f32> = Some(80.0 / 20.0);
    let pause_label_font_size = 0.015;

    // Container
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Pause),
        None,
        image_for_container,
        Vec3::new(x_anchor, 40.0, container_layer),
        20.0,
        Some(20.0 / 28.0),
        None,
    );

    // Label
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Pause),
        Some(Labels::Pause),
        None,
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
        commands,
        asset_server,
        window,
        Some(Buttons::Resume),
        Some(Containers::Pause),
        None,
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
        commands,
        asset_server,
        window,
        Some(Buttons::Settings),
        Some(Containers::Pause),
        None,
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
        commands,
        asset_server,
        window,
        Some(Buttons::MainMenu),
        Some(Containers::Pause),
        None,
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
        commands,
        asset_server,
        window,
        Some(Buttons::ExitGame),
        Some(Containers::Pause),
        None,
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
}

// DESPAWN_CONTAINER
// Used to close UI panels that have all of their elements associated with a specified container.
// If a UI component has a specific container type attached to it then you can delete it by using
// this function.
pub fn despawn_container(
    commands: &mut Commands,
    container: Containers,
    container_query: &Query<(Entity, &Containers)>,
) {
    for (entity, ui_container) in container_query.iter() {
        if *ui_container == container {
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
    interaction_query: Query<(&Interaction, &Buttons), Changed<Interaction>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    container_query: Query<(Entity, &Containers)>,
    mut commands: Commands,
    mut button_chain: ResMut<ButtonChain>,
    mut next_state: ResMut<NextState<State>>,
    mut state_history: ResMut<StateHistory>,
    mut app_exit: MessageWriter<AppExit>,
)
    -> Result<()>
{
    for (interaction, button) in interaction_query.iter() {

        if *interaction == Interaction::Pressed {

            match (button_chain.as_slice(), button) {

                ([Buttons::MainMenu], Buttons::Yes) => {
                    button_chain.clear();
                    despawn_container(&mut commands, Containers::Confirmation, &container_query);
                    state_history.clear();
                    next_state.set(State::MainMenu);
                },

                ([Buttons::ExitGame], Buttons::Yes) => {
                    button_chain.clear();
                    despawn_container(&mut commands, Containers::Confirmation, &container_query);
                    app_exit.write(AppExit::Success);
                },

                ([], Buttons::MainMenu) => {
                    let window = window_query.single()?;
                    spawn_confirmation(&mut commands, &asset_server, &window, "End the board and navigate to the Main Menu?");
                    button_chain.push(Buttons::MainMenu);
                },

                ([], Buttons::ExitGame) => {
                    let window = window_query.single()?;
                    spawn_confirmation(&mut commands, &asset_server, &window, "Close the program and exit the game?");
                    button_chain.push(Buttons::ExitGame);
                },

                (_, Buttons::No) => {
                    button_chain.clear();
                    despawn_container(&mut commands, Containers::Confirmation, &container_query);
                },

                (_, Buttons::Back) => {
                    button_chain.clear();
                    let previous_state = state_history.pop().unwrap_or(State::MainMenu);
                    next_state.set(previous_state);
                },

                (_, Buttons::Pause)   => {
                    let window = window_query.single()?;
                    spawn_pause(&mut commands, &asset_server, &window);
                },

                (_, Buttons::Resume)      => {
                    button_chain.clear();
                    despawn_container(&mut commands, Containers::Pause, &container_query);
                },

                (_, Buttons::Play)        => { button_chain.clear(); next_state.set(State::GameBoardCreator); },
                (_, Buttons::Settings)    => { button_chain.clear(); next_state.set(State::Settings); },
                (_, Buttons::CreateBoard) => { button_chain.clear(); next_state.set(State::GameBoard); },

                _ => { button_chain.clear(); }
            }
        }
    }

    Ok(())
}
