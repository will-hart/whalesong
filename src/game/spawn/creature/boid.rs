// Adapted from https://github.com/EmmettJayhart/bevy_boids/blob/main/src/lib.rs
// which is currently listed as "MIT or Apache-2.0" license in here
// https://github.com/EmmettJayhart/bevy_boids/blob/main/Cargo.toml
//
// Changes:
// - bevy 0.14 support,
// - separate gravity into a separate per entity component
// - make gravity a vec3 instead of assuming its always down

use bevoids::boids::{
    Boid, BoidSpace, BoidSpeed, BoidTurningStrength, BoidViewConfig, BoidsConfig, BoidsPlugin,
};
use bevy::prelude::*;

const BOID_MIN_SPEED: f32 = 50.0;
const BOID_MAX_SPEED: f32 = 100.0;

const BOID_COHESION: f32 = 0.3;
const BOID_SEPARATION: f32 = 0.03;
const BOID_ALIGNMENT: f32 = 0.09;
const BOID_BORDER_TURN_STRENGTH: f32 = 200.0;

const BOID_FOV: u32 = 240;
const BOID_VIEW_RANGE: f32 = 120.0;
const BOID_PROTECTED_RANGE: f32 = 12.0;

const BORDER_WIDTH: f32 = 2400.0;
const BORDER_HEIGHT: f32 = 1300.0;
const BORDER_MARGIN: f32 = 400.0;

pub fn get_default_boid() -> Boid {
    Boid::new(
        BoidSpeed::new(BOID_MIN_SPEED, BOID_MAX_SPEED),
        BoidTurningStrength::new(
            BOID_COHESION,
            BOID_SEPARATION,
            BOID_ALIGNMENT,
            BOID_BORDER_TURN_STRENGTH,
        ),
        BoidViewConfig::new(BOID_FOV, BOID_PROTECTED_RANGE, BOID_VIEW_RANGE),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BoidsPlugin).insert_resource(BoidsConfig {
        space: BoidSpace::TwoDimensional,
        debug: false,
    });
}
