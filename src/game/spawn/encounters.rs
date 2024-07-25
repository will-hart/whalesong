//! an encounter system

use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

#[derive(Event, Debug)]
pub struct SpawnEncounter {
    pub encounter_type: EncounterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncounterType {
    #[default]
    Bird,

    Fish,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_random_birds, spawn_random_fish).run_if(in_state(Screen::Playing)),
    );
}

fn spawn_random_birds(mut commands: Commands, time: Res<Time>, mut next_spawn: Local<f32>) {
    if *next_spawn > time.elapsed_seconds() {
        return;
    }

    info!("Bird spawn triggered");
    commands.trigger(SpawnEncounter {
        encounter_type: EncounterType::Bird,
    });

    let mut rng = rand::thread_rng();
    *next_spawn = time.elapsed_seconds() + rng.gen_range(15.0..35.0);
}

fn spawn_random_fish(mut commands: Commands, time: Res<Time>, mut next_spawn: Local<f32>) {
    if *next_spawn < 0.1 {
        *next_spawn = 2.0;
        return;
    }

    if *next_spawn > time.elapsed_seconds() {
        return;
    }

    info!("Fish school spawning");
    commands.trigger(SpawnEncounter {
        encounter_type: EncounterType::Fish,
    });

    let mut rng = rand::thread_rng();
    *next_spawn = time.elapsed_seconds() + rng.gen_range(5.0..15.0);
}
