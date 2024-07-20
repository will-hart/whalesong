use bevy::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

pub(crate) mod example;

pub trait GameStateMessage: Event + Serialize + DeserializeOwned {}

pub trait GameState<M: GameStateMessage>: Resource + Default {
    fn handle_message(&mut self, commands: &mut Commands, event: &M);
}

pub trait MessageObserver {
    fn observe_message<M: GameStateMessage, S: GameState<M>>(&mut self) -> &mut Self;
}

impl MessageObserver for App {
    fn observe_message<M, S>(&mut self) -> &mut Self
    where
        M: GameStateMessage,
        S: GameState<M>,
    {
        self.init_resource::<S>().observe(
            |trigger: Trigger<M>, mut commands: Commands, mut state: ResMut<S>| {
                state.handle_message(&mut commands, trigger.event());
            },
        );

        self
    }
}
