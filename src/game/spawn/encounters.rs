//! an encounter system

use bevy::prelude::*;

use crate::{game::weather::TravelDistance, screen::Screen};

#[derive(Event, Debug)]
pub struct SpawnEncounter {
    pub encounter_type: EncounterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncounterType {
    #[default]
    Bird,
    Fish,
    Ship,
    Iceberg,
    AdultWhale,
    BabyWhale,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_encounters.run_if(in_state(Screen::Playing)));
    app.init_resource::<EncounterTimers>();
}

#[derive(Resource)]
pub struct EncounterTimers {
    bird: f32,
    fish: f32,
    ship: f32,
    iceberg: f32,
    adult_whale: Option<f32>,
}

impl Default for EncounterTimers {
    fn default() -> Self {
        Self {
            bird: 12.,
            fish: 17.,
            ship: 30.,
            iceberg: 65.,
            adult_whale: None,
        }
    }
}

impl EncounterTimers {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Sets a time for an adult male to be spawned
    pub fn set_adult_spawn(&mut self, time: f32) {
        self.adult_whale = Some(time);
    }
}

fn spawn_encounters(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    mut encounters: ResMut<EncounterTimers>,
) {
    let now = distance.get();

    if encounters.bird < now {
        info!("Bird spawn triggered");
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Bird,
        });
        encounters.bird = distance.future_range(15.0..35.0);
    }

    if encounters.fish < now {
        info!("Fish school spawning");
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Fish,
        });
        encounters.fish = distance.future_range(5.0..15.0);
    }

    if encounters.ship < now {
        info!("Ship spawning");
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Ship,
        });
        encounters.ship = distance.future_range(25.0..55.0);
    }

    if encounters.iceberg < now {
        info!("Iceberg spawning");
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Iceberg,
        });
        encounters.iceberg = distance.future_range(10.0..15.0);
    }

    if encounters.adult_whale.is_some() && encounters.adult_whale.unwrap() < now {
        info!("Adult Whale spawning");
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::AdultWhale,
        });
        encounters.adult_whale = None; // only spawn one each flip
    }
}
