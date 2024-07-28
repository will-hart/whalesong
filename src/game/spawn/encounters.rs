//! an encounter system

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

use crate::{
    game::weather::{TravelDirection, TravelDistance},
    screen::Screen,
};

use std::ops::Range;

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

pub struct EncounterSpawnRate {
    slope: f32,
    intercept: Range<f32>,
}

impl EncounterSpawnRate {
    fn next_spawn(&self, t: f32) -> f32 {
        let mut rng = rand::thread_rng();
        let intercept = rng.gen_range(self.intercept.clone());
        t + ((t * self.slope / 100.) + intercept).clamp(1., 100.)
    }
}

pub type EncounterConfig = HashMap<TravelDirection, EncounterSpawnRate>;

#[derive(Resource)]
pub struct EncounterTimers {
    bird: f32,
    fish: f32,
    ship: f32,
    iceberg: f32,
    adult_whale: Option<f32>,

    bird_config: EncounterConfig,
    fish_config: EncounterConfig,
    ship_config: EncounterConfig,
    iceberg_config: EncounterConfig,
}

impl Default for EncounterTimers {
    fn default() -> Self {
        Self {
            bird: 12.,
            fish: 17.,
            ship: 45.,
            iceberg: 1.,
            adult_whale: None,

            bird_config: vec![
                (
                    TravelDirection::South,
                    EncounterSpawnRate {
                        slope: 0.,
                        intercept: 18.0..24.0,
                    },
                ),
                (
                    TravelDirection::North,
                    EncounterSpawnRate {
                        slope: 0.,
                        intercept: 18.0..22.0,
                    },
                ),
            ]
            .into_iter()
            .collect(),

            fish_config: vec![
                (
                    TravelDirection::South,
                    EncounterSpawnRate {
                        slope: 0.,
                        intercept: 12.0..22.0,
                    },
                ),
                (
                    TravelDirection::North,
                    EncounterSpawnRate {
                        slope: 0.,
                        intercept: 12.0..22.0,
                    },
                ),
            ]
            .into_iter()
            .collect(),

            ship_config: vec![
                (
                    TravelDirection::South,
                    EncounterSpawnRate {
                        slope: -24.,
                        intercept: 40.0..50.0,
                    },
                ),
                (
                    TravelDirection::North,
                    EncounterSpawnRate {
                        slope: -24.,
                        intercept: 40.0..50.0,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            iceberg_config: vec![
                (
                    TravelDirection::South,
                    EncounterSpawnRate {
                        slope: -67.,
                        intercept: 2.0..5.0,
                    },
                ),
                (
                    TravelDirection::North,
                    EncounterSpawnRate {
                        slope: 34.,
                        intercept: -7.0..-3.0,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        }
    }
}

impl EncounterTimers {
    pub fn reset(&mut self, direction: TravelDirection) {
        *self = Self::default();

        self.iceberg = match direction {
            TravelDirection::North => 1.0,
            TravelDirection::South => 65.0,
        };

        info!(
            "Travelling {direction:?}, first iceberg spawn at {}",
            self.iceberg
        );
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
    let direction = distance.travel_direction();

    if encounters.bird < now {
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Bird,
        });

        encounters.bird = encounters
            .bird_config
            .get(&direction)
            .unwrap()
            .next_spawn(now);

        info!(
            "Bird spawning at {now:.02} next bird at {:.02}",
            encounters.bird
        );
    }

    if encounters.fish < now {
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Fish,
        });

        encounters.fish = encounters
            .fish_config
            .get(&direction)
            .unwrap()
            .next_spawn(now);

        info!(
            "Fish spawning at {now:.02} next fish at {:.02}",
            encounters.fish
        );
    }

    if encounters.ship < now {
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Ship,
        });

        encounters.ship = encounters
            .ship_config
            .get(&direction)
            .unwrap()
            .next_spawn(now);

        if direction == TravelDirection::South {
            // stop spawning ships after 65s when travelling South
            if now > 65. {
                encounters.ship = f32::MAX;
            }
        }

        info!(
            "Ship spawning at {now:.02} next ship at {:.02}",
            encounters.ship
        );
    }

    if encounters.iceberg < now {
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::Iceberg,
        });

        encounters.iceberg = encounters
            .iceberg_config
            .get(&direction)
            .unwrap()
            .next_spawn(now);

        if direction == TravelDirection::North {
            // stop spawning icebergs after 40s when travelling north
            if now > 40. {
                encounters.iceberg = f32::MAX;
            }
        }

        info!(
            "Iceberg spawning at {now:.02} next iceberg at {:.02}",
            encounters.iceberg
        );
    }

    if encounters.adult_whale.is_some() && encounters.adult_whale.unwrap() < now {
        info!("Adult Whale spawning");
        commands.trigger(SpawnEncounter {
            encounter_type: EncounterType::AdultWhale,
        });

        encounters.adult_whale = None; // only spawn one each flip
    }
}
