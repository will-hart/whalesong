//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{screen::Screen, AppSet};

use super::spawn::player::{InputHelp, Whale, WhaleLocation};

/// how far the whale turns when pointing left or right (in radians)
const WHALE_MOVEMENT_SCALE: f32 = 0.4;
const WHALE_TURN_SPEED: f32 = 0.02;

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
        (despawn_out_of_view, rotate_whale_to_face_movement)
            .chain()
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

const WINDOW_BUFFER: f32 = 150.;

fn despawn_out_of_view(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    despawners: Query<(Entity, &Transform), With<DespawnWhenOutOfWindow>>,
) {
    let size = match windows.get_single() {
        Ok(w) => w.size(),
        Err(_) => return,
    };

    let half_size = size / 2.0;

    for (entity, transform) in &despawners {
        let position = transform.translation;

        if position.x < -half_size.x - WINDOW_BUFFER
            || position.x > half_size.x + WINDOW_BUFFER
            || position.y < -half_size.y - WINDOW_BUFFER
            || position.y > half_size.y + WINDOW_BUFFER
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
    // 1.3 is a magic number so the whale points where its going
}
