use bevy::prelude::*;

use crate::{
    screen::{Screen, UiFadeComplete, UiImageFadeInOut},
    ui::prelude::*,
    AppSet,
};

use super::assets::{HandleMap, ImageKey};

#[derive(Resource)]
pub struct IsFlipped(bool);

impl IsFlipped {
    pub fn get_flipped(&self) -> bool {
        self.0
    }

    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

#[derive(Component)]
pub struct Flippable;

#[derive(Event)]
pub struct DoFlip;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(IsFlipped(false));
    app.add_systems(
        Update,
        update_flip_timer
            .run_if(in_state(Screen::Playing))
            .in_set(AppSet::Update),
    );
    app.add_systems(PostUpdate, set_flippable_components.in_set(AppSet::Update)); // run in all states
    app.add_systems(OnExit(Screen::Playing), hard_reset_flippables);
    app.add_systems(OnEnter(Screen::Playing), set_up_flippables_for_gameplay);
    app.observe(perform_direction_switch_with_fade);

    #[cfg(debug_assertions)]
    app.add_systems(Update, debug_flip_system.run_if(in_state(Screen::Playing)));
}

#[cfg(debug_assertions)]
fn debug_flip_system(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyF) {
        commands.trigger(DoFlip);
    }
}

fn set_flippable_components(
    is_flipped: Res<IsFlipped>,
    mut flippables: Query<&mut Sprite, With<Flippable>>,
) {
    let flipped = is_flipped.get_flipped();

    for mut sprite in &mut flippables {
        sprite.flip_y = flipped;
    }
}

#[derive(Component)]
pub struct FlipTimer(Timer);

/// when its time to flip, fade out the audio and fade the camera to black
fn perform_direction_switch_with_fade(
    _trigger: Trigger<DoFlip>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    existing_flip_timers: Query<Entity, With<FlipTimer>>,
) {
    if !existing_flip_timers.is_empty() {
        warn!("Ignoring flip during existing flip");
        return;
    }

    commands.ui_root().with_children(|parent| {
        parent
            .spawn((
                Name::new("FadedBackground UI"),
                ImageBundle {
                    image: image_handles[&ImageKey::BlackPixel].clone_weak().into(),
                    style: Style {
                        width: Val::Vw(100.),
                        height: Val::Vh(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                UiImageFadeInOut::new(1.5, 5.),
            ))
            .observe(despawn_when_flip_complete);
    });

    // spawn a timer to handle the flip half way through the view transition
    commands.spawn((
        FlipTimer(Timer::from_seconds(2.5, TimerMode::Once)),
        StateScoped(Screen::Playing),
    ));
}

fn despawn_when_flip_complete(
    trigger: Trigger<UiFadeComplete>,
    mut commands: Commands,
    uis: Query<Entity, With<UiImageFadeInOut>>,
) {
    if let Ok(ui) = uis.get(trigger.entity()) {
        info!("Despawning flip black screen");
        commands.entity(ui).despawn();
    }
}

/// performs the actual flip. This usually happens half way through the fade out / fade in
/// of the UI black screen
fn update_flip_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut is_flipped: ResMut<IsFlipped>,
    mut timers: Query<(Entity, &mut FlipTimer)>,
    mut cameras: Query<&mut Transform, With<IsDefaultUiCamera>>,
) {
    for (entity, mut timer) in &mut timers {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();

            is_flipped.toggle();

            for mut camera in &mut cameras {
                camera.rotate(Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI));
            }
        }
    }
}

fn hard_reset_flippables(
    mut flipper: ResMut<IsFlipped>,
    mut cameras: Query<&mut Transform, With<IsDefaultUiCamera>>,
) {
    // this should be  enough to unflip all flippables in the next frame
    flipper.0 = false;

    for mut camera in &mut cameras {
        camera.rotation = Quat::default();
    }
}

fn set_up_flippables_for_gameplay(
    mut flipper: ResMut<IsFlipped>,
    mut cameras: Query<&mut Transform, With<IsDefaultUiCamera>>,
) {
    // this should be  enough to unflip all flippables in the next frame
    flipper.0 = true;

    for mut camera in &mut cameras {
        camera.rotation = Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI);
    }
}
