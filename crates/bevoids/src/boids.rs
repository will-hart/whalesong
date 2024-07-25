use bevy::prelude::{Component, Plugin, Resource, Update, Vec3};

/// The plugin you have to add to use boids.
///
/// Nothing special here :)
pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                systems::handle_boid_gizmos,
                systems::handle_boid_movement,
                systems::handle_boid_orientation,
            ),
        );
    }
}

/// The configuration resource of the boids.
///
/// This will be moved to a component later on
#[derive(Resource)]
pub struct BoidsConfig {
    pub space: BoidSpace,
    pub debug: bool,
}

/// Whether the boids are in 3d or 2d space
pub enum BoidSpace {
    TwoDimensional,
    ThreeDimensional,
}

/// Represents the turning strength for the different parts that make up the boid's
/// movement.
///
/// These parameters are very finnicky, so you might need to tweak a lot.
/// Refer to [this](https://en.wikipedia.org/wiki/Boids) to understand what
/// `coherence`, `separation` and `alignment` does.
#[derive(Clone, Copy)]
pub struct BoidTurningStrength {
    /// How strongly the boid steers towards the center of mass of flock.
    pub coherence: f32,
    /// How strongly the boid turns away from boids to close to it
    pub separation: f32,
    /// How strongly the boid aligns with the average heading of its flockmates
    pub alignment: f32,
    /// How strongly the boid turns away from [its borders](BoidBorder)
    pub border: f32,
}

impl BoidTurningStrength {
    pub fn new(coherence: f32, separation: f32, alignment: f32, border: f32) -> Self {
        Self {
            coherence,
            separation,
            alignment,
            border,
        }
    }
}

/// Represents the different options for view configuration of a boid
///
/// **NOTE**: FOV isn't currently implemented.
#[derive(Clone, Copy)]
pub struct BoidViewConfig {
    /// The field of view of a boid.
    pub fov: u32,
    /// The range that causes a boid to avoid another boid.
    /// Shouldn't be larger than `view_range`
    pub protected_range: f32,
    /// How far a boid can see.
    /// Shouldn't be smaller than `protected_range`
    pub view_range: f32,
}

impl BoidViewConfig {
    pub fn new(fov: u32, protected_range: f32, view_range: f32) -> Self {
        Self {
            fov,
            protected_range,
            view_range,
        }
    }
}

/// Represents the min/max speed limits of a boid
#[derive(Clone, Copy)]
pub struct BoidSpeed {
    min: f32,
    max: f32,
}

impl BoidSpeed {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

/// The actual boid component. Attach this to any entity that should act like a boid.
///
/// **NOTE**: This will take control of the entity's [Transform](bevy::prelude::Transform)
///
/// ## Example
/// ```rust
/// commands.spawn((
///    MaterialMesh2dBundle {
///       mesh: meshes.add(RegularPolygon::new(10.0, 3)).into(),
///       material: materials.add(ColorMaterial::from(Color::WHITE)),
///       transform: Transform::from_xyz(x as f32 * spacer, y as f32 * spacer, 0.0),
///       ..default()
///    },
///    Boid::new(
///       BoidSpeed::new(BOID_MIN_SPEED, BOID_MAX_SPEED),
///       BoidTurningStrength::new(
///          BOID_COHESION,
///          BOID_SEPARATION,
///          BOID_ALIGNMENT,
///          BOID_BORDER_TURN_STRENGTH,
///       ),
///       BoidViewConfig::new(BOID_FOV, BOID_PROTECTED_RANGE, BOID_VIEW_RANGE),
///    ),
/// ));
/// ```
///
/// If you want to fetch the boid's current velocity, use the [Boid::velocity] method.
#[derive(Component, Clone, Copy)]
pub struct Boid {
    /// The boid's min/max speed limits
    pub speed: BoidSpeed,
    /// The boid's turning strength parameters.
    /// These parameters are very finnicky. Stick to small numbers.
    ///
    /// To learn how to configure these values, refer to [this](https://en.wikipedia.org/wiki/Boids)
    pub turning_strength: BoidTurningStrength,
    /// The boid's view configuration.
    /// This includes FOV, View range and protected range.
    ///
    /// **NOTE**: FOV isn't currently implemented.
    pub view_config: BoidViewConfig,
    velocity: Vec3,
}

impl Boid {
    pub fn new(
        speed: BoidSpeed,
        turning_strength: BoidTurningStrength,
        view_config: BoidViewConfig,
    ) -> Self {
        Self {
            speed,
            turning_strength,
            view_config,
            velocity: Vec3::ZERO,
        }
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }
}

/// Component used for grouping boids. This effectively means that boids in
/// different groups ignore each other.
///
/// ## How to use
/// The collision group is used as a bitflag. This means one boid can have multiple
/// collision groups.
///
/// ```rust
/// // Will ignore boid 3
/// let boid1 = commands.spawn((
///    Boid::default(),
///    BoidCollisionGroup::GROUP_1
/// )).id();
///
/// // Will interact with both boids
/// let boid2 = commands.spawn((
///    Boid::default(),
///    BoidCollisionGroup::GROUP_1 | BoidCollisionGroup::GROUP_2
/// )).id();
///
/// // Will ignore boid 1
/// let boid3 = commands.spawn((
///    Boid::default(),
///    BoidCollisionGroup::GROUP_2
/// )).id();
/// ```
#[derive(Component, Clone, Copy)]
pub struct BoidCollisionGroup(u32);

bitflags::bitflags! {

    impl BoidCollisionGroup: u32 {
        /// The group n°1.
        const GROUP_1 = 1 << 0;
        /// The group n°2.
        const GROUP_2 = 1 << 1;
        /// The group n°3.
        const GROUP_3 = 1 << 2;
        /// The group n°4.
        const GROUP_4 = 1 << 3;
        /// The group n°5.
        const GROUP_5 = 1 << 4;
        /// The group n°6.
        const GROUP_6 = 1 << 5;
        /// The group n°7.
        const GROUP_7 = 1 << 6;
        /// The group n°8.
        const GROUP_8 = 1 << 7;
        /// The group n°9.
        const GROUP_9 = 1 << 8;
        /// The group n°10.
        const GROUP_10 = 1 << 9;
        /// The group n°11.
        const GROUP_11 = 1 << 10;
        /// The group n°12.
        const GROUP_12 = 1 << 11;
        /// The group n°13.
        const GROUP_13 = 1 << 12;
        /// The group n°14.
        const GROUP_14 = 1 << 13;
        /// The group n°15.
        const GROUP_15 = 1 << 14;
        /// The group n°16.
        const GROUP_16 = 1 << 15;
        /// The group n°17.
        const GROUP_17 = 1 << 16;
        /// The group n°18.
        const GROUP_18 = 1 << 17;
        /// The group n°19.
        const GROUP_19 = 1 << 18;
        /// The group n°20.
        const GROUP_20 = 1 << 19;
        /// The group n°21.
        const GROUP_21 = 1 << 20;
        /// The group n°22.
        const GROUP_22 = 1 << 21;
        /// The group n°23.
        const GROUP_23 = 1 << 22;
        /// The group n°24.
        const GROUP_24 = 1 << 23;
        /// The group n°25.
        const GROUP_25 = 1 << 24;
        /// The group n°26.
        const GROUP_26 = 1 << 25;
        /// The group n°27.
        const GROUP_27 = 1 << 26;
        /// The group n°28.
        const GROUP_28 = 1 << 27;
        /// The group n°29.
        const GROUP_29 = 1 << 28;
        /// The group n°30.
        const GROUP_30 = 1 << 29;
        /// The group n°31.
        const GROUP_31 = 1 << 30;
        /// The group n°32.
        const GROUP_32 = 1 << 31;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl Default for BoidCollisionGroup {
    fn default() -> Self {
        Self::ALL
    }
}

/// Represents the border that a boid should stay within
///
/// Each field is optional, to allow for any border you want!
///
/// The tuple in each field goes like this: (point, margin).
/// That means the first value is the actual point in space that the border is.
/// The margin is added onto that space, and basically chooses when the boid should start turning.
/// With a smaller margin, you should increase the strength of the [BoidTurningStrength] `border` field
#[derive(Component, Default)]
pub struct BoidBorder {
    pub top: Option<(f32, f32)>,
    pub bottom: Option<(f32, f32)>,
    pub left: Option<(f32, f32)>,
    pub right: Option<(f32, f32)>,
    pub front: Option<(f32, f32)>,
    pub back: Option<(f32, f32)>,
}

impl BoidBorder {
    fn calc_avoidance(&self, position: Vec3, boid: &Boid) -> Vec3 {
        // returns `point` normalized to [0, 1] range, allowing overflow for increased strength.
        fn normalize(point: f32, start: f32, end: f32) -> f32 {
            (point - start) / (end - start).clamp(0.0, f32::MAX)
        }

        let mut movement_vector = Vec3::ZERO;

        if let Some((left, margin)) = self.left {
            if position.x < left + margin {
                let normalized_point = 1.0 - normalize(position.x, left, left + margin);

                movement_vector.x += boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((right, margin)) = self.right {
            if position.x > right - margin {
                let normalized_point = normalize(position.x, right - margin, right);

                movement_vector.x -= boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((top, margin)) = self.top {
            if position.y > top - margin {
                let normalized_point = normalize(position.y, top - margin, top);

                movement_vector.y -= boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((bottom, margin)) = self.bottom {
            if position.y < bottom + margin {
                let normalized_point = 1.0 - normalize(position.y, bottom, bottom + margin);

                movement_vector.y += boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((front, margin)) = self.front {
            if position.z < front + margin {
                let normalized_point = 1.0 - normalize(position.z, front, front + margin);

                movement_vector.z += boid.turning_strength.border * normalized_point;
            }
        }
        if let Some((back, margin)) = self.back {
            if position.z > back - margin {
                let normalized_point = normalize(position.z, back - margin, back);

                movement_vector.z -= boid.turning_strength.border * normalized_point;
            }
        }

        movement_vector
    }
}

mod systems {
    use bevy::{
        prelude::{Color, Entity, Gizmos, Query, Res, Transform, Vec2},
        time::Time,
    };

    use super::*;

    pub(super) fn handle_boid_movement(
        time: Res<Time>,
        mut boid_query: Query<(
            &mut Transform,
            &mut Boid,
            Option<&BoidBorder>,
            Option<&BoidCollisionGroup>,
            Entity,
        )>,
    ) {
        let boids = boid_query
            .iter()
            .map(|(transform, boid, _, collision_group, entity)| {
                (*transform, *boid, collision_group.copied(), entity)
            })
            .collect::<Vec<_>>();

        for (mut transform, mut boid, border, collision_group, entity) in boid_query.iter_mut() {
            let mut movement_vector = Vec3::ZERO;

            let mut separation_vector = Vec3::ZERO;
            let mut average_velocity = Vec3::ZERO;
            let mut average_position = Vec3::ZERO;
            let mut neighbouring_boids = 0;

            for (
                neighbour_transform,
                neighbour_boid,
                neighbour_collision_group,
                neighbour_entity,
            ) in &boids
            {
                if *neighbour_entity == entity {
                    continue;
                }

                if let Some(collision_group) = collision_group {
                    if let Some(neighbour_collision_group) = neighbour_collision_group {
                        if !collision_group.intersects(*neighbour_collision_group) {
                            continue;
                        }
                    }
                }

                let distance = transform
                    .translation
                    .distance(neighbour_transform.translation);

                if distance <= boid.view_config.view_range {
                    if distance < boid.view_config.protected_range {
                        let normalized_diff =
                            (neighbour_transform.translation - transform.translation).normalize();
                        let strength = boid.view_config.protected_range - distance;

                        separation_vector -= normalized_diff * strength;
                    }

                    average_velocity += neighbour_boid.velocity;
                    average_position += neighbour_transform.translation;
                    neighbouring_boids += 1;
                }
            }

            if neighbouring_boids > 0 {
                average_velocity /= neighbouring_boids as f32;
                average_position /= neighbouring_boids as f32;

                movement_vector +=
                    (average_velocity - boid.velocity) * boid.turning_strength.alignment;
                movement_vector +=
                    (average_position - transform.translation) * boid.turning_strength.coherence;
            }

            movement_vector += separation_vector * boid.turning_strength.separation;

            if let Some(border) = border {
                movement_vector += border.calc_avoidance(transform.translation, &boid);
            }

            let new_velocity = boid.velocity + movement_vector;

            boid.velocity = if new_velocity.length_squared() > 0.0 {
                new_velocity.clamp_length(boid.speed.min, boid.speed.max)
            } else {
                Vec3::X * boid.speed.min
            };

            transform.translation += boid.velocity * time.delta_seconds();
        }
    }

    pub(super) fn handle_boid_orientation(
        mut boid_query: Query<(&mut Transform, &Boid)>,
        config: Res<BoidsConfig>,
    ) {
        for (mut transform, boid) in boid_query.iter_mut() {
            let forward = transform.forward();

            match config.space {
                BoidSpace::TwoDimensional => transform.look_to(*forward, boid.velocity.normalize()),
                BoidSpace::ThreeDimensional => {
                    transform.look_to(boid.velocity.normalize(), *forward)
                }
            }
        }
    }

    pub(super) fn handle_boid_gizmos(
        config: Res<BoidsConfig>,
        boid_query: Query<(&Transform, &Boid)>,
        mut gizmos: Gizmos,
    ) {
        if config.debug {
            match config.space {
                BoidSpace::TwoDimensional => {
                    for (transform, boid) in boid_query.iter() {
                        for angle in
                            -(boid.view_config.fov as i32 / 2)..boid.view_config.fov as i32 / 2
                        {
                            let angle_radians = (angle as f32).to_radians();

                            gizmos.ray_2d(
                                transform.translation.truncate(),
                                transform
                                    .up()
                                    .truncate()
                                    .rotate(Vec2::from_angle(angle_radians))
                                    * boid.view_config.view_range,
                                Color::srgba(1.0, 0.0, 0.0, 0.1),
                            );
                        }
                    }
                }
                BoidSpace::ThreeDimensional => {
                    for (transform, boid) in boid_query.iter() {
                        for angle in
                            -(boid.view_config.fov as i32) / 2..boid.view_config.fov as i32 / 2
                        {
                            let angle_radians = (angle as f32).to_radians();

                            gizmos.ray(
                                transform.translation,
                                transform
                                    .up()
                                    .truncate()
                                    .rotate(Vec2::from_angle(angle_radians))
                                    .extend(0.0)
                                    * boid.view_config.view_range,
                                Color::srgba(1.0, 0.0, 0.0, 0.1),
                            );
                        }
                    }
                }
            }
        }
    }
}
