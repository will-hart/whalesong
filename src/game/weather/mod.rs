//! Adds weather effects to the game.

use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

mod day_night_cycle;
mod rain;
mod waves;

pub use day_night_cycle::TintWithDayNightCycle;
pub use waves::Wave;

/// How far the whale has travelled
#[derive(Resource)]
pub struct TravelDistance(f32);

impl TravelDistance {
    /// Gets the current travel distance (in seconds because physics)
    pub fn get(&self) -> f32 {
        self.0
    }

    /// Generates a future time within `now + range`
    pub fn future_range(&self, range: Range<f32>) -> f32 {
        self.0 + rand::thread_rng().gen_range(range)
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TravelDistance(0.));

    app.add_systems(OnEnter(Screen::Playing), reset_travel_distance);

    app.add_systems(
        Update,
        (update_travel_distance).run_if(in_state(Screen::Playing)),
    );

    app.add_plugins((day_night_cycle::plugin, rain::plugin, waves::plugin));
}

fn reset_travel_distance(mut distance: ResMut<TravelDistance>) {
    distance.0 = 0.;
}

fn update_travel_distance(time: Res<Time>, mut distance: ResMut<TravelDistance>) {
    distance.0 += time.delta_seconds();
}
