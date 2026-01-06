#![allow(clippy::type_complexity)]

use crate::{
    GameMusic, GameState, SoundEnabled, cleanup, click_sound, settings::write_sound_setting,
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

const TRANSPARENT: Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);

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
                        resource_changed::<SoundEnabled>.and(not(resource_added::<SoundEnabled>)),
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MenuEntity,
        ))
        .with_children(|parent| {
            let logo = asset_server.load("sprites/logo.png");
            parent.spawn((
                ImageNode::new(logo),
                Node {
                    height: Val::Percent(30.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    ..default()
                },
            ));

            for (action, text) in [
                (MenuButtonAction::Play, "PLAY"),
                (MenuButtonAction::HighScores, "HIGHSCORES"),
                (MenuButtonAction::Help, "HELP"),
            ] {
                parent
                    .spawn((Button, BackgroundColor(TRANSPARENT), action))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(text),
                            TextFont {
                                font: asset_server.load("fonts/Retroville NC.ttf"),
                                font_size: 40.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            TextLayout::new_with_justify(Justify::Center),
                        ));
                    });
            }

            parent
                .spawn((
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        ..default()
                    },
                    BackgroundColor(TRANSPARENT),
                    MenuButtonAction::SoundToggle,
                ))
                .with_children(|parent| {
                    let path = if sound_enabled.0 {
                        "sprites/soundOn.png"
                    } else {
                        "sprites/soundOff.png"
                    };
                    let icon = asset_server.load(path);
                    parent.spawn((ImageNode::new(icon), SoundButton));
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut music_query: Query<&mut AudioSink, With<GameMusic>>,
    mut sound_enabled: ResMut<SoundEnabled>,
    mut sound_button_query: Query<(Entity, &mut ImageNode), With<SoundButton>>,
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
                    if let Ok(mut sink) = music_query.single_mut() {
                        sink.toggle_mute();
                    } else {
                        commands.spawn((
                            AudioPlayer::<AudioSource>(asset_server.load("audio/music.ogg")),
                            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.1)),
                            GameMusic,
                        ));
                    }
                    sound_enabled.0 = !sound_enabled.0;

                    let (_, mut ui_image) = sound_button_query.single_mut().unwrap();
                    let path = if sound_enabled.0 {
                        "sprites/soundOn.png"
                    } else {
                        "sprites/soundOff.png"
                    };
                    *ui_image = ImageNode::new(asset_server.load(path));

                    write_sound_setting(sound_enabled.0);
                }
            }
        }
    }
}
