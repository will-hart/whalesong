//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::{
    flipper::IsFlipped,
    spawn::{
        player::{InputHelp, Whale, WhaleArrivalMarker, WhaleRotation},
        WindowSize,
    },
};

/// How fast the whale travels around the screen
pub const WHALE_TRAVEL_SPEED: f32 = 0.4; // magic number

/// how far the whale turns when pointing left or right (in radians)
const WHALE_MOVEMENT_SCALE: f32 = 0.6;
const WHALE_TURN_SPEED: f32 = 0.02;

pub const WHALE_SCREEN_BUFFER_FRACTION: f32 = 0.3;

/// Moves towards the given location and triggers an "ArrivedAtLocation"
/// event on the entity when it arrives, removing this component
#[derive(Component)]
pub struct MoveTowardsLocation {
    pub target: Vec3,
    pub speed: f32,
}

#[derive(Event)]
pub struct ArrivedAtLocation;

/// Moves the entity with the given velocity. Useful for things like waves
#[derive(Component)]
pub struct MoveWithVelocity(pub Vec3);

/// Denotes a component that rotates to face the direction of travel
/// This is done in the [`move_towards_location`] system.
#[derive(Component)]
pub struct RotateToFaceMovement;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(Update, update_movement_intent.in_set(AppSet::RecordInput));

    // Apply movement based on controls.
    app.register_type::<DespawnWhenOutOfWindow>();
    app.add_systems(
        FixedUpdate,
        (
            despawn_out_of_view,
            move_whale,
            move_towards_location,
            move_with_velocity,
        )
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    pub intent: Vec2,
    pub action: bool,
}

#[derive(Event)]
pub struct PlayerActionRequested;

fn update_movement_intent(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    is_flipped: Res<IsFlipped>,
    mut controller_query: Query<&mut MovementController>,
    helpers: Query<Entity, With<InputHelp>>,
) {
    let mut intent = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    if is_flipped.get_flipped() {
        // move the whale "up and down" the screen. Note that the key changes when the screen
        // is flipped but as its just the camera being rotated, the direction doesn't
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            intent.y = -1.0; // we can only move "down" the screen. the whale naturally drifts back up
                             // when no keys are pressed in the "move whale" system below
        }
    } else {
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            intent.y = -1.0; // we can only move "down" the screen. the whale naturally drifts back up
                             // when no keys are pressed in the "move whale" system below
        }
    }

    if intent.x.abs() > 0.01 {
        for entity in &helpers {
            commands.entity(entity).despawn();
        }
    }

    if is_flipped.get_flipped() {
        intent.x = -intent.x;
    }

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
        if input.just_pressed(KeyCode::Space) {
            commands.trigger(PlayerActionRequested);
        }
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
            // info!("Despawning {entity:?}");
            commands.entity(entity).despawn();
        }
    }
}

fn move_whale(
    mut whale_pos: ResMut<WhaleRotation>,
    win_size: Res<WindowSize>,
    movements: Query<&MovementController>,
    // don't move whales that are being "moved to location" or are arriving
    mut whales: Query<
        &mut Transform,
        (
            With<Whale>,
            Without<WhaleArrivalMarker>,
            Without<MoveTowardsLocation>,
        ),
    >,
) {
    let movement = movements.single();
    let delta_move = if movement.intent.length_squared() < 0.1 {
        // no input, drift the whale back up the screen slowly
        Vec2::new(0., 0.35)
    } else {
        movement.intent.normalize_or_zero()
    } * WHALE_MOVEMENT_SCALE;

    if let Ok(mut whale) = whales.get_single_mut() {
        whale.translation = win_size.clamp_to_screen_with_buffer(
            whale.translation + delta_move.extend(0.),
            Val::Percent(WHALE_SCREEN_BUFFER_FRACTION * 100.),
        );

        whale_pos.target_rotation = if movement.intent.y < 0. {
            movement.intent.x * 0.5 * WHALE_MOVEMENT_SCALE
        } else {
            movement.intent.x * WHALE_MOVEMENT_SCALE
        };
        whale_pos.current_rotation = whale_pos
            .current_rotation
            .lerp(whale_pos.target_rotation, WHALE_TURN_SPEED);

        whale.rotation =
            Quat::from_axis_angle(Vec3::new(0., 0., 1.), whale_pos.current_rotation * 1.3);
    }
}

/// Moves entities that have the [`MoveTowardsLocation`] component towards their location
fn move_towards_location(
    mut commands: Commands,
    mut movers: Query<(
        Entity,
        &Name,
        &mut Transform,
        &MoveTowardsLocation,
        Option<&RotateToFaceMovement>,
    )>,
) {
    for (entity, name, mut mover, details, rotate_to_face) in &mut movers {
        let prev = mover.translation;
        mover.translation = mover
            .translation
            .move_towards(details.target, details.speed);
        mover.translation.z = prev.z; // keep z-height

        if rotate_to_face.is_some() {
            let direction = (prev - mover.translation).xy().normalize();
            mover.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.));
        }

        if (mover.translation - details.target).length_squared() < 1.0 {
            info!("{name:?} {entity} has arrived, removing MoveOnTarget and triggering event");
            commands.entity(entity).remove::<MoveTowardsLocation>();
            commands.trigger_targets(ArrivedAtLocation, entity);
        }
    }
}

/// Moves entities that have the [`MoveWithVelocity`] component in their direction of travel
fn move_with_velocity(
    mut movers: Query<(
        &mut Transform,
        &MoveWithVelocity,
        Option<&RotateToFaceMovement>,
    )>,
) {
    for (mut mover, details, rotate_to_face) in &mut movers {
        let prev = mover.translation;
        mover.translation += details.0;
        mover.translation.z = prev.z; // keep z-height

        if rotate_to_face.is_some() {
            let direction = (prev - mover.translation).xy().normalize();
            mover.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.));
        }
    }
}
