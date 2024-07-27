use bevy::prelude::*;
use ui_palette::WHALE_BLUE;

use crate::{
    screen::{Screen, UiFadeComplete, UiImageFadeInOut},
    ui::prelude::*,
    AppSet,
};

use super::{
    spawn::{
        creature::Creature,
        encounters::EncounterTimers,
        player::{Whale, WhaleRotation},
        WindowSize,
    },
    weather::{Precipitation, Raininess, TravelDistance, WeatherState, INITIAL_TIME_OF_DAY},
};

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
pub struct DoFlip {
    pub flip_text: String,
}

#[derive(Event)]
pub struct FlipComplete;

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
        commands.trigger(DoFlip {
            flip_text: "Debug Flip".into(),
        });
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
    trigger: Trigger<DoFlip>,
    mut commands: Commands,
    existing_flip_timers: Query<Entity, With<FlipTimer>>,
) {
    if !existing_flip_timers.is_empty() {
        warn!("Ignoring flip during existing flip");
        return;
    }

    let fade = UiImageFadeInOut::new(1.0, 3.0);

    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(WHALE_BLUE),
                style: Style {
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            fade,
        ))
        .observe(despawn_when_flip_complete)
        .with_children(|parent| {
            parent.spawn((
                Name::new("Label Text"),
                TextBundle {
                    text: Text::from_section(
                        trigger.event().flip_text.clone(),
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    background_color: BackgroundColor(Color::srgba(0., 0., 0., 0.)),
                    ..Default::default()
                },
            ));
        });

    // spawn a timer to handle the flip half way through the view transition
    commands.spawn((
        FlipTimer(Timer::from_seconds(1.5, TimerMode::Once)),
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
        commands.entity(ui).despawn_recursive();
    }
}

/// performs the actual flip. This usually happens half way through the fade out / fade in
/// of the UI black screen. Probably should break all this arguments up lol
fn update_flip_timer(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WindowSize>,
    mut is_flipped: ResMut<IsFlipped>,
    mut distance: ResMut<TravelDistance>,
    mut encounters: ResMut<EncounterTimers>,
    mut raininess: ResMut<Raininess>,
    mut weather: ResMut<WeatherState>,
    mut whale_rot: ResMut<WhaleRotation>,
    creatures: Query<Entity, With<Creature>>,
    rain: Query<Entity, With<Precipitation>>,
    mut whales: Query<&mut Transform, With<Whale>>,
    mut timers: Query<(Entity, &mut FlipTimer)>,
    mut cameras: Query<&mut Transform, (With<IsDefaultUiCamera>, Without<Whale>)>,
) {
    for (entity, mut timer) in &mut timers {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // despawn the timer
            commands.entity(entity).despawn();

            // reset all the flippin' state
            is_flipped.toggle();
            distance.reset_timer();
            encounters.reset();
            raininess.reset();
            weather.time_of_day = INITIAL_TIME_OF_DAY;

            // update whale position
            whale_rot.current_rotation = 0.;
            whale_rot.target_rotation = 0.;

            let half = win_size.half();
            for mut whale in &mut whales {
                whale.translation = Vec3::new(0., 0.8 * half.y, 0.0);
                whale.rotation = Quat::IDENTITY;
            }

            // despawn all creatures
            for creature in &creatures {
                commands.entity(creature).despawn_recursive();
            }

            for rain in &rain {
                commands.entity(rain).despawn();
            }

            // rotate the camera
            for mut camera in &mut cameras {
                camera.rotate(Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI));
            }

            // notify others that the flip is complete
            commands.trigger(FlipComplete);
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
