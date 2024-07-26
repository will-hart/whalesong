use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{assets::SfxKey, audio::sfx::PlaySfx},
    screen::Screen,
};

use super::TravelDistance;

#[derive(Resource, Debug)]
pub struct Raininess {
    factor: f32,
    time_rain_ends: f32,
}

#[derive(Event)]
pub struct RainChanged {
    pub is_raining: bool,
}

const RAIN_THRESHOLD: f32 = 0.5;

impl Raininess {
    /// updates the rain status and returns true if the status changed
    pub fn update(&mut self, distance: &TravelDistance, delta: Range<f32>) -> bool {
        let mut rng = rand::thread_rng();
        self.factor = (self.factor + rng.gen_range(delta)).clamp(0.0, 1.0);
        let was_raining = self.is_raining();

        if self.factor > RAIN_THRESHOLD && self.time_rain_ends < 0.01 {
            // start raining
            self.time_rain_ends = distance.future_range(12.0..15.0); // rain for this many seconds
        } else if self.time_rain_ends > 0. && self.time_rain_ends < distance.get() {
            // stop raining
            self.reset();
        }

        self.is_raining() != was_raining
    }

    pub fn is_raining(&self) -> bool {
        self.time_rain_ends > 0.
    }

    pub fn factor(&self) -> f32 {
        self.factor
    }

    pub fn reset(&mut self) {
        self.factor = 0.;
        self.time_rain_ends = 0.;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Raininess {
        factor: 0.,
        time_rain_ends: 0.,
    });
    app.add_systems(OnEnter(Screen::Playing), reset_raininess);
    app.add_systems(Update, update_raininess.run_if(in_state(Screen::Playing)));
    app.observe(handle_rain_changed);
}

pub fn reset_raininess(mut raininess: ResMut<Raininess>) {
    raininess.reset();
}

pub fn update_raininess(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    mut raininess: ResMut<Raininess>,
) {
    if raininess.update(&distance, -0.001..0.0015) {
        commands.trigger(RainChanged {
            is_raining: raininess.is_raining(),
        });
        warn!(
            "Raininess: {}, {} raining",
            raininess.factor(),
            if raininess.is_raining() { "is" } else { "not" }
        );
    }
}

#[derive(Component)]
pub struct Rain;

fn handle_rain_changed(
    trigger: Trigger<RainChanged>,
    mut commands: Commands,
    rains: Query<Entity, With<Rain>>,
) {
    let evt = trigger.event();

    if evt.is_raining {
        info!("Spawning rain");
        let entity = commands.spawn(Rain).id();
        commands.trigger(
            PlaySfx::once(SfxKey::RainAmbient)
                .with_parent(entity)
                .with_volume(2.5),
        );
    } else {
        info!("Despawning rain");
        for rain in &rains {
            commands.entity(rain).despawn();
        }
    }
}
