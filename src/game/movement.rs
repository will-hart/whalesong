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
    pub remove_on_arrival: bool,
}

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
        (despawn_out_of_view, move_whale, move_towards_location)
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

fn update_movement_intent(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
    helpers: Query<Entity, With<InputHelp>>,
) {
    // Collect directional input. `y` is -1, because if we aren't pressing anything the whale
    // should drift back to the top of the screen slowly
    let mut intent = Vec2::new(0., 0.25);
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y = -1.0;
    }

    if intent.x.abs() > 0.01 {
        for entity in &helpers {
            commands.entity(entity).despawn();
        }
    }

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
        controller.action = input.just_pressed(KeyCode::Space);
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
    mut whale_pos: ResMut<WhaleLocation>,
    win_size: Res<WindowSize>,
    movements: Query<&MovementController>,
    mut whales: Query<&mut Transform, (With<Whale>, Without<MoveTowardsLocation>)>, // don't move whales that are being "moved to location"
) {
    let movement = movements.single();
    let delta_move = movement.intent.normalize_or_zero() * WHALE_MOVEMENT_SCALE;

    if let Ok(mut whale) = whales.get_single_mut() {
        whale.translation = win_size.clamp_to_screen_with_buffer(
            whale.translation + delta_move.extend(0.),
            Val::Percent(20.),
        );

        whale_pos.target_rotation = movement.intent.x * WHALE_MOVEMENT_SCALE;
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

        if rotate_to_face.is_some() {
            let direction = (prev - mover.translation).xy().normalize();
            mover.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.));
        }

        if details.remove_on_arrival && (mover.translation - details.target).length_squared() < 1.0
        {
            info!("{name:?} {entity} has arrived, removing MoveOnTarget");
            commands.entity(entity).remove::<MoveTowardsLocation>();
        }
    }
}
