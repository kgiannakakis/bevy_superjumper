#![allow(clippy::type_complexity)]

use crate::{
    cleanup, click_sound, settings::write_sound_setting, GameMusic, GameState, SoundEnabled,
};
use bevy::{audio::Volume, prelude::*};

#[derive(Component)]
struct MenuEntity;

#[derive(Component)]
struct SoundButton;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    HighScores,
    Help,
    SoundToggle,
}

const TRANSPARENT: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                OnExit(GameState::Menu),
                (click_sound, cleanup::<MenuEntity>),
            )
            .add_systems(
                Update,
                (
                    menu_action,
                    click_sound.run_if(
                        resource_changed::<SoundEnabled>
                            .and_then(not(resource_added::<SoundEnabled>)),
                    ),
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_enabled: ResMut<SoundEnabled>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .with_children(|parent| {
            let logo = asset_server.load("sprites/logo.png");
            parent.spawn(ImageBundle {
                style: Style {
                    height: Val::Percent(30.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    ..default()
                },
                image: UiImage::new(logo),
                ..default()
            });

            for (action, text) in [
                (MenuButtonAction::Play, "PLAY"),
                (MenuButtonAction::HighScores, "HIGHSCORES"),
                (MenuButtonAction::Help, "HELP"),
            ] {
                parent
                    .spawn((
                        ButtonBundle {
                            background_color: TRANSPARENT.into(),
                            ..default()
                        },
                        action,
                    ))
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: asset_server.load("fonts/Retroville NC.ttf"),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_justify(JustifyText::Center),
                        );
                    });
            }

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        background_color: TRANSPARENT.into(),
                        ..default()
                    },
                    MenuButtonAction::SoundToggle,
                ))
                .with_children(|parent| {
                    let path = if sound_enabled.0 {
                        "sprites/soundOn.png"
                    } else {
                        "sprites/soundOff.png"
                    };
                    let icon = asset_server.load(path);
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(icon),
                            ..default()
                        },
                        SoundButton,
                    ));
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    music_query: Query<&AudioSink, With<GameMusic>>,
    mut sound_enabled: ResMut<SoundEnabled>,
    mut sound_button_query: Query<(Entity, &mut UiImage), With<SoundButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => game_state.set(GameState::Playing),
                MenuButtonAction::HighScores => game_state.set(GameState::HighScores),
                MenuButtonAction::Help => game_state.set(GameState::Help),
                MenuButtonAction::SoundToggle => {
                    if let Ok(sink) = music_query.get_single() {
                        sink.toggle();
                    } else {
                        commands.spawn((
                            AudioBundle {
                                source: asset_server.load("audio/music.ogg"),
                                settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.1)),
                            },
                            GameMusic,
                        ));
                    }
                    sound_enabled.0 = !sound_enabled.0;

                    let (_, mut ui_image) = sound_button_query.single_mut();
                    let path = if sound_enabled.0 {
                        "sprites/soundOn.png"
                    } else {
                        "sprites/soundOff.png"
                    };
                    *ui_image = UiImage::new(asset_server.load(path));

                    write_sound_setting(sound_enabled.0);
                }
            }
        }
    }
}
