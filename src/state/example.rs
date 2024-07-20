use bevy::{prelude::*, state::state::OnEnter};
use serde::{Deserialize, Serialize};

use crate::screen::Screen;

use super::{GameState, GameStateMessage, MessageObserver};

#[derive(Clone, Debug, Event, Default, Serialize, Deserialize)]
pub enum ExampleMessage {
    #[default]
    Noop,
    RngSeed {
        seed: f64,
    },
}

impl GameStateMessage for ExampleMessage {}

#[derive(Resource, Default)]
pub struct ExampleGameState {
    events: Vec<ExampleMessage>,
}

impl GameState<ExampleMessage> for ExampleGameState {
    fn handle_message(&mut self, _commands: &mut bevy::prelude::Commands, event: &ExampleMessage) {
        println!("HANDLING MESSAGE: {event:?}");
        self.events.push(event.clone());
    }
}

pub fn plugin(app: &mut App) {
    app.observe_message::<ExampleMessage, ExampleGameState>()
        .add_systems(OnEnter(Screen::Playing), send_game_message);
}

fn send_game_message(mut cmd: Commands) {
    cmd.trigger(ExampleMessage::RngSeed { seed: 1234.0 });
}
