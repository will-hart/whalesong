//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{screen::Screen, AppSet};

use super::spawn::player::{InputHelp, Whale};

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
        Update,
        (
            despawn_out_of_view,
            move_to_y_pos,
            rotate_whale_to_face_movement,
        )
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

fn despawn_out_of_view(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    despawners: Query<(Entity, &Transform), With<DespawnWhenOutOfWindow>>,
) {
    let height = match windows.get_single() {
        Ok(w) => w.height() + 100.0,
        Err(_) => return,
    };
    let half_height = height / 2.0;

    for (entity, transform) in &despawners {
        let position = transform.translation.y;

        // only need to check one way here as these things are moving "up" the screen
        if position > half_height {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct MoveToY {
    pub y: f32,
    pub speed: f32,
}

fn move_to_y_pos(
    mut commands: Commands,
    time: Res<Time>,
    mut movers: Query<(Entity, &MoveToY, &mut Transform)>,
) {
    for (entity, mover, mut transform) in &mut movers {
        let delta_y = mover.y - transform.translation.y;
        if delta_y.abs() < 0.5 {
            commands.entity(entity).remove::<MoveToY>();
            continue;
        }

        // we're moving down so "delta_y" should always be negative
        let movement = (-mover.speed * time.delta_seconds()).max(delta_y);
        transform.translation.y += movement;
    }
}

fn rotate_whale_to_face_movement(
    movements: Query<&MovementController>,
    mut whales: Query<&mut Transform, With<Whale>>,
) {
    let movement = movements.single();

    for mut whale in &mut whales {
        whale.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), movement.0.x * 0.4);
    }
}
