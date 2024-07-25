use bevoids::prelude::*;
use bevy::prelude::*;

const BOID_MIN_SPEED: f32 = 300.0;
const BOID_MAX_SPEED: f32 = 700.0;

const BOID_COHESION: f32 = 0.05;
const BOID_SEPARATION: f32 = 0.4;
const BOID_ALIGNMENT: f32 = 0.14;
const BOID_BORDER_TURN_STRENGTH: f32 = 50.0;

const BOID_FOV: u32 = 240;
const BOID_VIEW_RANGE: f32 = 120.0;
const BOID_PROTECTED_RANGE: f32 = 60.0;

const BORDER_WIDTH: f32 = 1200.0;
const BORDER_HEIGHT: f32 = 700.0;
const BORDER_MARGIN: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoidsPlugin))
        .insert_resource(BoidsConfig {
            space: BoidSpace::ThreeDimensional,
            debug: false,
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 150.0,
        })
        .add_systems(Startup, setup)
        .run()
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            -BORDER_WIDTH / 1.1,
            BORDER_HEIGHT / 1.1,
            -BORDER_WIDTH / 1.1,
        )
        .looking_at(Vec3::new(0.0, -BORDER_HEIGHT / 2.0, 0.0), Vec3::Y),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes
            .add(Plane3d::default().mesh().size(BORDER_WIDTH, BORDER_WIDTH))
            .into(),
        material: materials.add(StandardMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_xyz(0.0, -BORDER_HEIGHT / 2.0, 0.0),
        ..default()
    });

    for x in -5..5 {
        for y in -5..5 {
            for z in -5..5 {
                let spacer = 10.0;

                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(7.5, 7.5, 20.0)).into(),
                        material: materials.add(StandardMaterial::from(Color::RED)),
                        transform: Transform::from_xyz(
                            x as f32 * spacer,
                            y as f32 * spacer,
                            z as f32 * spacer,
                        ),
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
                        front: Some((-BORDER_WIDTH / 2.0, BORDER_MARGIN)),
                        back: Some((BORDER_WIDTH / 2.0, BORDER_MARGIN)),
                    },
                ));
            }
        }
    }
}
