
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::spawns::*;

pub struct StatesForGUI {}
impl Plugin for StatesForGUI {
    fn build(&self, app: &mut App) {

        // Initializing a state type into the application.
        app.init_state::<GUIState>();

        // UI State Enter and Exit Definitions
        // These "states" are the GUI elements that players engage with - main menu, settings, the gameboard, the gameboard creator, and the pause menu.
        // Each state has its own enter functionality to spawn appropriate entities related to the GUI.
        // Each state has its own exit functionality to despawn entities present before transitioning to the next room.
        app.add_systems(OnEnter(GUIState::MainMenu_Default), setup_main_menu_default);
        app.add_systems(OnExit(GUIState::MainMenu_Default), cleanup_gui_entities);
        app.add_systems(OnEnter(GUIState::Settings_Default), setup_settings_default);
        app.add_systems(OnExit(GUIState::Settings_Default), cleanup_gui_entities);
        app.add_systems(OnEnter(GUIState::GameBoardCreator_Default), setup_gameboard_creator_default);
        app.add_systems(OnExit(GUIState::GameBoardCreator_Default), cleanup_gui_entities);
        app.add_systems(OnEnter(GUIState::GameBoard_Default), setup_gameboard_default);
        app.add_systems(OnExit(GUIState::GameBoard_Default), cleanup_gui_entities);
        app.add_systems(OnEnter(GUIState::PauseMenu_Default), setup_pause_menu_default);
        app.add_systems(OnExit(GUIState::PauseMenu_Default), cleanup_gui_entities);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GUIState {
    #[default]
    MainMenu_Default,
    Settings_Default,
    GameBoardCreator_Default,
    GameBoard_Default,
    PauseMenu_Default
}


fn setup_main_menu_default
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_settings_default
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_gameboard_default
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_gameboard_creator_default
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn setup_pause_menu_default
(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
    -> Result<()>
{
    let window: &Window = window_query.single()?;

    Ok(())
}

fn cleanup_gui_entities
(
    mut commands: Commands,
    entity_query: Query<Entity, With<ActiveEntity>>,
)
{
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
