use bevoids::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const BOID_MIN_SPEED: f32 = 400.0;
const BOID_MAX_SPEED: f32 = 1000.0;

const BOID_COHESION: f32 = 0.1;
const BOID_SEPARATION: f32 = 0.4;
const BOID_ALIGNMENT: f32 = 0.14;
const BOID_BORDER_TURN_STRENGTH: f32 = 200.0;

const BOID_FOV: u32 = 240;
const BOID_VIEW_RANGE: f32 = 120.0;
const BOID_PROTECTED_RANGE: f32 = 60.0;

const BORDER_WIDTH: f32 = 2400.0;
const BORDER_HEIGHT: f32 = 1300.0;
const BORDER_MARGIN: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoidsPlugin))
        .insert_resource(BoidsConfig {
            space: BoidSpace::TwoDimensional,
            debug: false,
        })
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut cam_bundle = Camera2dBundle::default();
    cam_bundle.projection.scale = 3.0;

    commands.spawn(cam_bundle);

    for x in -10..10 {
        for y in -10..10 {
            let spacer = 10.0;

            let (group, material) = if x > 0 {
                (
                    BoidCollisionGroup::GROUP_1,
                    materials.add(ColorMaterial::from(Color::RED)),
                )
            } else {
                (
                    BoidCollisionGroup::GROUP_2,
                    materials.add(ColorMaterial::from(Color::CYAN)),
                )
            };

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(RegularPolygon::new(10.0, 3)).into(),
                    material,
                    transform: Transform::from_xyz(x as f32 * spacer, y as f32 * spacer, 0.0),
                    ..default()
                },
                Boid::new(
                    BoidSpeed::new(BOID_MIN_SPEED, BOID_MAX_SPEED),
                    BoidTurningStrength::new(
                        BOID_COHESION,
                        BOID_SEPARATION,
                        BOID_ALIGNMENT,
                        BOID_BORDER_TURN_STRENGTH,
                    ),
                    BoidViewConfig::new(BOID_FOV, BOID_PROTECTED_RANGE, BOID_VIEW_RANGE),
                ),
                BoidBorder {
                    left: Some((-BORDER_WIDTH / 2.0, BORDER_MARGIN)),
                    right: Some((BORDER_WIDTH / 2.0, BORDER_MARGIN)),
                    top: Some((BORDER_HEIGHT / 2.0, BORDER_MARGIN)),
                    bottom: Some((-BORDER_HEIGHT / 2.0, BORDER_MARGIN)),
                    front: None,
                    back: None,
                },
                group,
            ));
        }
    }
}
