//! The title screen that appears when the game starts.

use bevy::prelude::*;
use ui_palette::NODE_BACKGROUND;

use super::Screen;
use crate::{
    game::{
        assets::{HandleMap, ImageKey, SoundtrackKey},
        audio::soundtrack::PlaySoundtrack,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut clear_colour: ResMut<ClearColor>,
) {
    clear_colour.0 = NODE_BACKGROUND;

    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|logo_parent| {
                    logo_parent.spawn(ImageBundle {
                        image: image_handles[&ImageKey::Logo].clone_weak().into(),
                        ..default()
                    });
                });

            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .image_button(image_handles[&ImageKey::PlayButton].clone_weak())
                        .insert(TitleAction::Play);
                    children
                        .image_button(image_handles[&ImageKey::CreditsButton].clone_weak())
                        .insert(TitleAction::Credits);

                    #[cfg(not(target_family = "wasm"))]
                    children
                        .image_button(image_handles[&ImageKey::ExitButton].clone_weak())
                        .insert(TitleAction::Exit);
                });
        });

    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Menu));
}

fn handle_title_action(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => {
                    next_screen.set(Screen::Playing);
                    commands.trigger(PlaySoundtrack::Disable);
                }
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
