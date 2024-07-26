use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::game::assets::{HandleMap, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
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
            child.spawn(bundle);
        });
    } else {
        commands.spawn(bundle);
    }
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub struct PlaySfx {
    pub key: SfxKey,
    pub looped: bool,
    pub parent_entity: Option<Entity>,
    pub volume: f32,
}

impl PlaySfx {
    pub fn once(key: SfxKey) -> Self {
        Self {
            key,
            looped: false,
            parent_entity: None,
            volume: 1.0,
        }
    }

    pub fn looped(key: SfxKey) -> Self {
        Self {
            key,
            looped: true,
            parent_entity: None,
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
}
