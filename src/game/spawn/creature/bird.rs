use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        movement::{MoveTowardsLocation, WHALE_TRAVEL_SPEED},
        spawn::{player::WhaleLocation, WindowSize},
    },
    screen::Screen,
};

use super::get_creature_path;

pub const BIRD_SPEED: f32 = WHALE_TRAVEL_SPEED * 1.2;

/// Used to indicate a curious creature, such as a bird
#[derive(Component)]
pub struct Curious {
    until: f32,
}

/// Denotes birds that can not become curious (they may have already been curious, or may just be immune as some birds are)
#[derive(Component)]
pub struct Incurious;

/// Denotes a bird
#[derive(Component)]
pub struct Bird;

/// Added when a bird is transitioning between curious and incurious

#[derive(Component)]
pub struct LosingCuriosity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (gain_curiosity, lose_curiosity).run_if(in_state(Screen::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        (scale_curious_birds, return_to_flying_off).run_if(in_state(Screen::Playing)),
    );
}

pub const BIRD_CURIOSITY_THRESHOLD: f32 = 120.;

/// Looks at birds
fn gain_curiosity(
    mut commands: Commands,
    time: Res<Time>,
    whale_pos: Res<WhaleLocation>,
    birds: Query<(Entity, &Transform), (With<Bird>, Without<Curious>, Without<Incurious>)>,
) {
    if birds.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    let target = Vec3::new(
        rng.gen_range(-20.0..20.0),
        whale_pos.y + rng.gen_range(-20.0..20.0),
        0.,
    );

    for (bird, tx) in &birds {
        if (tx.translation - target).length_squared()
            < BIRD_CURIOSITY_THRESHOLD * BIRD_CURIOSITY_THRESHOLD
        {
            info!("bird {bird:?} is curious");
            commands.entity(bird).insert((
                Curious {
                    // marks them for the curiosity AI system
                    until: time.elapsed_seconds() + rng.gen_range(10.0..25.0),
                },
                Incurious, // prevents the bird from becoming curious again
                MoveTowardsLocation {
                    target,
                    speed: BIRD_SPEED,
                    remove_on_arrival: true,
                },
            )); // stops them from moving off the screen and instead circles them
        }
    }
}

fn scale_curious_birds(mut birds: Query<&mut Transform, With<Curious>>) {
    for mut tx in &mut birds {
        // update the scale
        let scale = (tx.scale.x - 0.001).clamp(0.45, 1.0);
        tx.scale = Vec3::splat(scale);
    }
}

/// After the timer expires, mark the birds as losing curiosity
fn lose_curiosity(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    time: Res<Time>,
    birds: Query<(Entity, &Curious), (With<Bird>, Without<LosingCuriosity>)>,
) {
    let size = win_size.size();
    for (bird, curious) in &birds {
        if curious.until <= time.elapsed_seconds() {
            // update the entity so it navigates back off the screen
            let (_, target) = get_creature_path(size, 64.);

            info!("bird {bird:?} losing curiosity");
            commands
                .entity(bird)
                .insert(LosingCuriosity)
                .remove::<Curious>()
                .insert(MoveTowardsLocation {
                    target,
                    speed: BIRD_SPEED,
                    remove_on_arrival: true,
                });
        }
    }
}

fn return_to_flying_off(
    mut commands: Commands,
    mut birds: Query<
        (Entity, &mut Transform, &mut MoveTowardsLocation),
        (With<Bird>, With<LosingCuriosity>),
    >,
) {
    for (bird, mut tx, mut mover) in &mut birds {
        // slowly scale the bird up
        let splat = tx.scale.x + 0.003; // not sure why 0.001 doesn't work here, very confusing
        tx.scale = Vec3::splat(splat);

        // slowly speed up the bird
        mover.speed = (mover.speed + 0.001).min(BIRD_SPEED);

        // check if the bird is ready to leave
        if tx.scale.x >= 1.0 {
            info!("Bird {bird} is flying off");
            tx.scale = Vec3::ONE;
            commands.entity(bird).remove::<LosingCuriosity>();
        }
    }
}