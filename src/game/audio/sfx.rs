use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::game::assets::{HandleMap, SfxKey};

#[derive(Component, Copy, Clone)]
pub struct FadeIn {
    pub final_volume: f32,
    pub rate_per_second: f32,
}

#[derive(Component)]
pub struct FadeOut {
    pub rate_per_second: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx)
        .add_systems(Update, (fade_in, fade_out));
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let request = trigger.event();
    let sfx_key = request.key;

    let bundle = AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: if request.looped {
                PlaybackMode::Loop
            } else {
                PlaybackMode::Despawn
            },
            volume: Volume::new(request.volume),
            ..default()
        },
    };

    if let Some(parent) = request.parent_entity {
        commands.entity(parent).with_children(|child| {
            let mut child_cmds = child.spawn(bundle);
            if let Some(fade_in) = request.fade_in {
                child_cmds.insert(fade_in);
            }
        });
    } else {
        let mut cmds = commands.spawn(bundle);
        if let Some(fade_in) = request.fade_in {
            cmds.insert(fade_in);
        }
    }
}

fn fade_out(
    mut commands: Commands,
    time: Res<Time>,
    faders: Query<(Entity, &AudioSink, &FadeOut)>,
) {
    for (entity, fader, details) in &faders {
        let next_volume = fader.volume() - details.rate_per_second * time.delta_seconds();
        if next_volume <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        fader.set_volume(next_volume);
    }
}

fn fade_in(mut commands: Commands, time: Res<Time>, faders: Query<(Entity, &AudioSink, &FadeIn)>) {
    for (entity, fader, details) in &faders {
        let mut next_volume = fader.volume() + details.rate_per_second * time.delta_seconds();
        if next_volume >= details.final_volume {
            next_volume = details.final_volume;
            commands.entity(entity).remove::<FadeIn>();
        }

        fader.set_volume(next_volume);
    }
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub struct PlaySfx {
    pub key: SfxKey,
    pub looped: bool,
    pub parent_entity: Option<Entity>,
    fade_in: Option<FadeIn>,
    pub volume: f32,
}

impl PlaySfx {
    pub fn once(key: SfxKey) -> Self {
        Self {
            key,
            looped: false,
            parent_entity: None,
            fade_in: None,
            volume: 1.0,
        }
    }

    pub fn looped(key: SfxKey) -> Self {
        Self {
            key,
            looped: true,
            parent_entity: None,
            fade_in: None,
            volume: 1.0,
        }
    }

    pub fn with_parent(mut self, entity: Entity) -> Self {
        self.parent_entity = Some(entity);
        self
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn with_fade_in(mut self, fade: FadeIn) -> Self {
        self.fade_in = Some(fade);
        self
    }
}
