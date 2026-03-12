
use bevy::prelude::*;
use crate::spawns::UIButtons;
use crate::states_ui::UIState;

pub struct GameResources {}
impl Plugin for GameResources {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonChain>();
        app.init_resource::<UIStateHistory>();
    }
}

// ---------------------------------------------------------------------------------------------- //
// UI BUTTON CHAINING

#[derive(Resource, Default)]
pub struct ButtonChain {
    chain: Vec<UIButtons>,
}

impl ButtonChain {

    // Add button to the chain.
    pub fn push(&mut self, button: UIButtons) {
        self.chain.push(button);
    }

    // Clear the entire chain.
    pub fn clear(&mut self) {
        self.chain.clear();
    }

    // Return the chain as a slice so that it can be utilized in match statements.
    pub fn as_slice(&self) -> &[UIButtons] {
        self.chain.as_slice()
    }
}
// ---------------------------------------------------------------------------------------------- //



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
