// Adapted from https://github.com/EmmettJayhart/bevy_boids/blob/main/src/lib.rs
// which is currently listed as "MIT or Apache-2.0" license in here
// https://github.com/EmmettJayhart/bevy_boids/blob/main/Cargo.toml
//
// Changes:
// - bevy 0.14 support,
// - separate gravity into a separate per entity component
// - make gravity a vec3 instead of assuming its always down

use bevy::{prelude::*, utils::HashMap};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (apply_intent, apply_physics)
            .chain()
            .run_if(in_state(Screen::Playing)),
    )
    .init_resource::<BoidDescriptor>();
    app.register_type::<BoidDescriptor>();
}

#[derive(Resource, Reflect)]
pub struct BoidDescriptor {
    pub thrust: f32,
    pub lift: f32,
    pub bank: f32,
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub bank_rate: f32,
    pub rise_rate: f32,
    pub maximum_vision: f32,
}

impl Default for BoidDescriptor {
    fn default() -> Self {
        Self {
            thrust: 12.0,
            lift: 11.0,
            bank: 11.0,
            separation: 70.0,
            alignment: 1.0,
            cohesion: 10.0,
            bank_rate: 0.01,
            rise_rate: 0.01,
            maximum_vision: 240.0,
        }
    }
}

#[derive(Component, Reflect)]
pub struct Boid;

/// Allows specifying bird specific gravity
#[derive(Component, Reflect)]
pub struct BoidGravity(pub Vec3);

fn apply_intent(
    mut boids_query: Query<(Entity, &mut Transform, &GlobalTransform), With<Boid>>,
    mut headings: Local<HashMap<Entity, (f32, f32)>>,
    descriptor: Res<BoidDescriptor>,
    time: Res<Time>,
) {
    for (boid, transform, global_transform) in boids_query.iter() {
        let mut heading = Vec3::ZERO;

        for (other_boid, _, other_global_transform) in boids_query.iter() {
            if other_boid == boid {
                continue;
            }

            let position = global_transform.translation();
            let other_position = other_global_transform.translation();
            let distance = position.distance(other_position);

            if distance > descriptor.maximum_vision {
                continue;
            }

            let separation =
                (position - other_position).normalize_or_zero() * descriptor.separation / distance;
            let alignment = other_global_transform.forward() * descriptor.alignment;
            let cohesion = (other_position - position).normalize_or_zero() * descriptor.cohesion;

            heading += separation + alignment + cohesion;
        }

        let lateral = descriptor.bank_rate
            * (heading.dot(transform.left().as_vec3()) + transform.up().dot(Vec3::Y));
        let vertical = descriptor.rise_rate
            * (heading.dot(transform.up().as_vec3()) + transform.up().dot(Vec3::Y));

        headings.insert(boid, (lateral, vertical));
    }

    for (boid, mut transform, _) in boids_query.iter_mut() {
        let &(lateral, vertical) = headings.get(&boid).unwrap_or(&(0.0, 0.0));

        transform.rotate_local_z(lateral * time.delta_seconds());
        transform.rotate_local_x(vertical * time.delta_seconds());
    }
}

fn apply_physics(
    mut boids_query: Query<(&mut Transform, Option<&BoidGravity>), With<Boid>>,
    descriptor: Res<BoidDescriptor>,
    time: Res<Time>,
) {
    for (mut transform, gravity) in boids_query.iter_mut() {
        let thrust = transform.forward() * descriptor.thrust;
        let lift = transform.up() * descriptor.lift;

        let mut applied =
            (thrust + lift + gravity.map(|g| g.0).unwrap_or(Vec3::ZERO)) * time.delta_seconds();
        applied.z = 0.;

        transform.translation += applied;
        info!(
            "FISH: {:?} -> {:?}",
            applied.truncate(),
            transform.translation.truncate()
        );

        let bank = transform.right().dot(Vec3::Y) * descriptor.bank;
        transform.rotate_y(bank * time.delta_seconds());
    }
}
