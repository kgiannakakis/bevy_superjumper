use bevy::prelude::*;

use crate::{
    highscores::{check_and_update_highscores, HighScores},
    GameState,
};

use super::{GameEntity, PlayState, Points};

#[derive(Component)]
pub(super) struct GameUi;

#[derive(Component)]
pub(super) struct GameButtonUi;

#[derive(Component)]
pub(super) struct ScoreUi;

#[derive(Component)]
pub(super) struct GameOverUi;

#[derive(Component)]
pub(super) enum PlayButtonAction {
    Play,
    Resume,
    Quit,
    Pause,
}

const TRANSPARENT: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

pub(super) fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the game UI
    commands
        .spawn((
            GameEntity,
            GameUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            for (action, text, visibility) in [
                (PlayButtonAction::Play, "READY?", Visibility::Visible),
                (PlayButtonAction::Resume, "RESUME", Visibility::Hidden),
                (PlayButtonAction::Quit, "QUIT", Visibility::Hidden),
            ] {
                parent
                    .spawn((
                        ButtonBundle {
                            background_color: TRANSPARENT.into(),
                            visibility,
                            ..default()
                        },
                        GameButtonUi,
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
                            top: Val::Px(10.0),
                            right: Val::Px(10.0),
                            ..default()
                        },
                        background_color: TRANSPARENT.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    GameButtonUi,
                    PlayButtonAction::Pause,
                ))
                .with_children(|parent| {
                    let path = "sprites/pause.png";
                    let icon = asset_server.load(path);
                    parent.spawn(ImageBundle {
                        image: UiImage::new(icon),
                        ..default()
                    });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "SCORE: 0",
                            TextStyle {
                                font: asset_server.load("fonts/Retroville NC.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_text_justify(JustifyText::Left),
                        ScoreUi,
                    ));
                });
        });
}

pub(super) fn spawn_game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_ui_query: Query<Entity, With<GameUi>>,
    points: Res<Points>,
    mut high_scores: ResMut<HighScores>,
) {
    for entity in game_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let score = points.0;
    let score_title = if check_and_update_highscores(&mut high_scores, score) {
        format!("NEW HIGHSCORE: {}", score)
    } else {
        format!("SCORE: {}", score)
    };

    commands
        .spawn((
            GameEntity,
            GameOverUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            score_title,
                            TextStyle {
                                font: asset_server.load("fonts/Retroville NC.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_text_justify(JustifyText::Left),
                        ScoreUi,
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "GAME OVER",
                        TextStyle {
                            font: asset_server.load("fonts/Retroville NC.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_text_justify(JustifyText::Left),));
                });
        });
}

pub(super) fn update_buttons_visibility(
    play_state: Res<State<PlayState>>,
    mut visibility_query: Query<(Entity, &mut Visibility), With<GameButtonUi>>,
    mut score_visibility_query: Query<
        (Entity, &mut Visibility),
        (With<ScoreUi>, Without<GameButtonUi>),
    >,
) {
    let actions = [
        PlayButtonAction::Play,
        PlayButtonAction::Resume,
        PlayButtonAction::Quit,
        PlayButtonAction::Pause,
    ];
    for (action_index, (_, mut visibility)) in (&mut visibility_query).into_iter().enumerate() {
        match actions[action_index] {
            PlayButtonAction::Play => {
                *visibility = if *play_state == PlayState::Ready {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            }
            PlayButtonAction::Resume => {
                *visibility = if *play_state == PlayState::Paused {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            }
            PlayButtonAction::Quit => {
                *visibility = if *play_state == PlayState::Paused {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            }
            PlayButtonAction::Pause => {
                *visibility = if *play_state == PlayState::Running {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            }
        }
    }

    *score_visibility_query.single_mut().1 = if *play_state != PlayState::Ready {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

pub(super) fn ui_action(
    interaction_query: Query<
        (&Interaction, &PlayButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut play_state: ResMut<NextState<PlayState>>,
    points: Res<Points>,
    mut high_scores: ResMut<HighScores>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PlayButtonAction::Play => play_state.set(PlayState::Running),
                PlayButtonAction::Resume => play_state.set(PlayState::Running),
                PlayButtonAction::Quit => {
                    check_and_update_highscores(&mut high_scores, points.0);
                    play_state.set(PlayState::Ready);
                    game_state.set(GameState::Menu);
                }
                PlayButtonAction::Pause => play_state.set(PlayState::Paused),
            }
        }
    }
}

pub(super) fn update_score_text(mut query: Query<&mut Text, With<ScoreUi>>, points: Res<Points>) {
    for mut text in &mut query {
        text.sections[0].value = format!("SCORE: {}", points.0);
    }
}

pub(super) fn go_back_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    play_state.set(PlayState::Ready);
    game_state.set(GameState::Menu);
}
