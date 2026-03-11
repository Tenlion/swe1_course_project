
use bevy::prelude::*;
use crate::states_ui::UIState;

pub struct GameResources {}
impl Plugin for GameResources {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIStateHistory::default());
    }
}

// ---------------------------------------------------------------------------------------------- //
// UI STATE HISTORY

const HISTORY_CAP: usize = 10;

#[derive(Resource, Default)]
pub struct UIStateHistory {
    stack: Vec<UIState>,
}

impl UIStateHistory {

    // Adding to history.  Will remove oldest state in history when cap has been reached to allow
    // for new additions to the state history.
    pub fn push(&mut self, state: UIState) {
        if self.stack.len() >= HISTORY_CAP {
            self.stack.remove(0);
        }
        self.stack.push(state);
    }

    // Removing/getting latest history.
    // Because we're working with a stack we must return an option for the scenario that the stack
    // could be empty.  Realistically speaking, I don't think this would ever happen since players
    // start on the main menu and always move into another UI.
    pub fn pop(&mut self) -> Option<UIState> {
        self.stack.pop()
    }

    // Wiping the full UI state history.
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
// ---------------------------------------------------------------------------------------------- //
