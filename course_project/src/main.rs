
// Module Declarations
mod states;
mod spawns;
mod resources;

// Imports
use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::resources::Resources;
use crate::states::States;
use crate::spawns::Spawns;

fn main() {

    App::new()

        // Plugins!  Which are also just files that we make and can add to a game in order
        // to condense and organize code for projects within Bevy.
        .add_plugins(DefaultPlugins)
        .add_plugins(Resources{})
        .add_plugins(States{})
        .add_plugins(Spawns{})

        // Sets the window mode and camera.
        .add_systems(Startup, setup)

        // Tells the app to launch.
        .run();
}


fn setup(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
)
    -> Result<()>
{

    let mut window = windows.single_mut()?;
    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
    window.resizable = false;

    let camera_position = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);

    commands.spawn((
        Camera2d,
        Transform::from_translation(camera_position)
    ));

    Ok(())
}


pub fn spawn_camera
(
    commands: &mut Commands,
    position: Vec3,
)
{
    commands.spawn((
        Camera2d,
        Transform::from_translation(position)
    ));
}
