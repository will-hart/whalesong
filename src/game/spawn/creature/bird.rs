use bevy::prelude::*;
use rand::Rng;
use tiny_bail::rq;

use crate::{
    game::{
        animation::SpriteAnimationPlayer,
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::PlaySfx,
        flipper::Flippable,
        movement::{MoveTowardsLocation, MoveWithVelocity, WHALE_TRAVEL_SPEED},
        spawn::{encounters::EncounterType, player::Whale, WindowSize},
    },
    screen::Screen,
};

use super::{get_creature_path, Creature};

pub const BIRD_SPEED: f32 = WHALE_TRAVEL_SPEED * 1.2;

/// Used to indicate a curious creature, such as a bird
#[derive(Component)]
pub struct Curious {
    pub until: f32,
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
        (
            scale_curious_birds,
            curious_birds_follow_whale,
            return_to_flying_off,
        )
            .run_if(in_state(Screen::Playing)),
    );
}

/// Spawns a bird when `SpawnEncounter(Bird)` is triggered. Called by the parent creature plugin
pub(super) fn spawn(
    commands: &mut Commands,
    win_size: Vec2,
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_animation = SpriteAnimationPlayer::bird();

    let (from_pos, to_pos) = get_creature_path(win_size, 64.);
    let mut rng = rand::thread_rng();

    let mut entity_cmds = commands.spawn((
        Name::new("Bird"),
        Creature(EncounterType::Bird),
        Bird,
        SpriteBundle {
            texture: image_handles[&ImageKey::Creatures].clone_weak(),
            transform: Transform::from_translation(from_pos + Vec3::Z * 1.5), // spawn above ships
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        Flippable,
        StateScoped(Screen::Playing),
        MoveWithVelocity((to_pos - from_pos).normalize() * BIRD_SPEED),
    ));

    // some birds are just incurious
    if rng.gen_bool(0.3) {
        entity_cmds.insert(Incurious);
    }
}

pub const BIRD_CURIOSITY_THRESHOLD: f32 = 120.;

/// Looks at birds and works out if they're close enough to a whale to get curious about it
fn gain_curiosity(
    mut commands: Commands,
    time: Res<Time>,
    whales: Query<&Transform, With<Whale>>,
    birds: Query<(Entity, &Transform), (With<Bird>, Without<Curious>, Without<Incurious>)>,
) {
    if birds.is_empty() {
        return;
    }

    let whale = rq!(whales.get_single());
    let mut rng = rand::thread_rng();
    let target = Vec3::new(
        whale.translation.x + rng.gen_range(-20.0..20.0),
        whale.translation.y + rng.gen_range(-20.0..20.0),
        0.,
    );

    for (bird, tx) in &birds {
        let delta_pos = target - tx.translation;
        if delta_pos.length_squared() < BIRD_CURIOSITY_THRESHOLD * BIRD_CURIOSITY_THRESHOLD {
            info!("bird {bird:?} is curious");
            commands.trigger(PlaySfx::once(SfxKey::Gull));

            commands.entity(bird).insert((
                Curious {
                    // marks them for the curiosity AI system
                    until: time.elapsed_seconds() + rng.gen_range(10.0..25.0),
                },
                Incurious, // prevents the bird from becoming curious again
                MoveWithVelocity(delta_pos.normalize_or_zero() * BIRD_SPEED),
            ));
        }
    }
}

fn curious_birds_follow_whale(
    whales: Query<&Transform, With<Whale>>,
    mut birds: Query<
        (&Transform, &mut MoveWithVelocity),
        (With<Bird>, With<Curious>, Without<LosingCuriosity>),
    >,
) {
    if birds.is_empty() {
        return;
    }

    let whale = rq!(whales.get_single());
    let mut rng = rand::thread_rng();

    for (bird_tx, mut bird) in &mut birds {
        bird.0 = (Vec3::new(
            whale.translation.x + rng.gen_range(-20.0..20.0),
            whale.translation.y + rng.gen_range(-20.0..20.0),
            0.,
        ) - bird_tx.translation)
            .normalize_or_zero()
            * BIRD_SPEED;
    }
}

fn scale_curious_birds(mut birds: Query<&mut Transform, (With<Bird>, With<Curious>)>) {
    for mut tx in &mut birds {
        // update the scale
        let scale = (tx.scale.x - 0.001).clamp(0.65, 1.0);
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
        mover.speed = (mover.speed + 0.0001).min(BIRD_SPEED);

        // check if the bird is ready to leave
        if tx.scale.x >= 1.0 {
            info!("Bird {bird} is flying off");
            tx.scale = Vec3::ONE;
            commands.entity(bird).remove::<LosingCuriosity>();
        }
    }
}
