//! Adds weather effects to the game.

use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{game::flipper::DoFlip, screen::Screen};

mod day_night_cycle;
mod rain;
mod waves;

pub use day_night_cycle::TintWithDayNightCycle;
pub use day_night_cycle::{WeatherState, INITIAL_TIME_OF_DAY};
pub use rain::Raininess;
pub use waves::Wave;

/// The distance at which flipping occurs
pub const DISTANCE_FLIPPING: f32 = 120.;

const FLIP_MESSAGES: [&str; 2] = [
    "Our whale arrived in the warm Northern waters for winter.",
    "Our whale arrived in cooler Southern waters for the summer.",
];

/// How far the whale has travelled
#[derive(Resource, Default)]
pub struct TravelDistance {
    distance: f32,
    total_distance: f32,
    num_flips: u32,
    is_flipping: bool,
}

impl TravelDistance {
    /// Gets the current travel distance (in seconds because physics)
    pub fn get(&self) -> f32 {
        self.distance
    }

    /// Generates a future time within `now + range`
    pub fn future_range(&self, range: Range<f32>) -> f32 {
        self.distance + rand::thread_rng().gen_range(range)
    }

    /// resets the distance travelled to 0
    pub fn reset_timer(&mut self) {
        self.distance = 0.;
        self.num_flips += 1;
    }

    /// Updates the travel distance and returns `true` if the distance
    /// should be flipped
    pub fn update(&mut self, delta: f32) -> bool {
        let prev = self.distance;

        self.distance += delta;
        self.total_distance += delta;

        let was_flipping = self.is_flipping;
        self.is_flipping = self.distance % DISTANCE_FLIPPING < prev;

        self.is_flipping && self.is_flipping != was_flipping
    }

    pub fn get_message(&self) -> String {
        FLIP_MESSAGES[(self.num_flips % 2) as usize].to_owned()
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TravelDistance::default());

    app.add_systems(OnEnter(Screen::Playing), reset_travel_distance);

    app.add_systems(
        Update,
        update_travel_distance.run_if(in_state(Screen::Playing)),
    );

    app.add_plugins((day_night_cycle::plugin, rain::plugin, waves::plugin));
}

fn reset_travel_distance(mut distance: ResMut<TravelDistance>) {
    *distance = TravelDistance::default();
}

fn update_travel_distance(
    mut commands: Commands,
    time: Res<Time>,
    mut distance: ResMut<TravelDistance>,
) {
    if distance.update(time.delta_seconds()) {
        commands.trigger(DoFlip {
            flip_text: distance.get_message(),
        });
    }
}
