//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::spawn::{
    player::{InputHelp, Whale, WhaleLocation},
    WindowSize,
};

/// How fast the "whale travels", i.e. how fast to move things past the whale
pub const WHALE_TRAVEL_SPEED: f32 = 0.45; // magic number

/// how far the whale turns when pointing left or right (in radians)
const WHALE_MOVEMENT_SCALE: f32 = 0.4;
const WHALE_TURN_SPEED: f32 = 0.02;

#[derive(Component)]
pub struct MoveTowardsLocation {
    pub target: Vec3,
    pub speed: f32,
}

/// Denotes an entity that moves as the whale moves, i.e. waves
#[derive(Component)]
pub struct MoveWithWhale;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<DespawnWhenOutOfWindow>();
    app.add_systems(
        FixedUpdate,
        (
            despawn_out_of_view,
            rotate_whale_to_face_movement,
            (
                update_mover_target_based_on_whale_movement,
                move_towards_location,
            )
                .chain(),
        )
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn record_movement_controller(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
    helpers: Query<Entity, With<InputHelp>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    if intent.x.abs() > 0.01 {
        for entity in &helpers {
            commands.entity(entity).despawn();
        }
    }

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnWhenOutOfWindow;

pub const WINDOW_DESPAWN_BUFFER: f32 = 150.;

fn despawn_out_of_view(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    despawners: Query<(Entity, &Transform), With<DespawnWhenOutOfWindow>>,
) {
    let half_size = win_size.half();

    for (entity, transform) in &despawners {
        let position = transform.translation;

        if position.x < -half_size.x - WINDOW_DESPAWN_BUFFER
            || position.x > half_size.x + WINDOW_DESPAWN_BUFFER
            || position.y < -half_size.y - WINDOW_DESPAWN_BUFFER
            || position.y > half_size.y + WINDOW_DESPAWN_BUFFER
        {
            info!("Despawning {entity:?}");
            commands.entity(entity).despawn();
        }
    }
}

fn rotate_whale_to_face_movement(
    mut whale_pos: ResMut<WhaleLocation>,
    movements: Query<&MovementController>,
    mut whales: Query<&mut Transform, With<Whale>>,
) {
    let movement = movements.single();

    whale_pos.target_rotation = movement.0.x * WHALE_MOVEMENT_SCALE;
    whale_pos.current_rotation = whale_pos
        .current_rotation
        .lerp(whale_pos.target_rotation, WHALE_TURN_SPEED);

    let mut whale = whales.single_mut();
    whale.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), whale_pos.current_rotation * 1.3);
    // 1.4 is a magic number so the whale points where its going
}

/// Moves entities that have the [`MoveTowardsLocation`] component towards their location
fn move_towards_location(mut movers: Query<(&mut Transform, &MoveTowardsLocation)>) {
    for (mut mover, details) in &mut movers {
        mover.translation = mover
            .translation
            .move_towards(details.target, details.speed);
    }
}

/// When the whale is moving, updates any entity with [`MovesWithWhale`] component
fn update_mover_target_based_on_whale_movement(
    whale_pos: Res<WhaleLocation>,
    mut movers: Query<(&mut Transform, &mut MoveTowardsLocation), With<MoveWithWhale>>,
) {
    for (mut tx, mut mover) in &mut movers {
        let delta = Vec3::new(-whale_pos.current_rotation * 4.3, 0.0, 0.0) * WHALE_TRAVEL_SPEED;

        // we need to move both the current pos and the target pos so that we don't end up with weird angles
        mover.target += delta;
        tx.translation += delta;
    }
}
