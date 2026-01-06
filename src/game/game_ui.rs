use bevy::prelude::*;

use crate::{
    GameState,
    highscores::{HighScores, check_and_update_highscores},
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

const TRANSPARENT: Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);

pub(super) fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the game UI
    commands
        .spawn((
            GameEntity,
            GameUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
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
                        Button,
                        BackgroundColor(TRANSPARENT),
                        visibility,
                        GameButtonUi,
                        action,
                    ))
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
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        height: Val::Px(64.0),
                        width: Val::Px(64.0),
                        ..default()
                    },
                    Visibility::Hidden,
                    GameButtonUi,
                    PlayButtonAction::Pause,
                ))
                .with_children(|parent| {
                    let path = "sprites/pause.png";
                    let icon = asset_server.load(path);
                    parent.spawn(ImageNode::new(icon));
                });

            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..default()
                    },
                    Visibility::Hidden,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SCORE: 0"),
                        TextFont {
                            font: asset_server.load("fonts/Retroville NC.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(Justify::Left),
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
        commands.entity(entity).despawn();
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(score_title),
                        TextFont {
                            font: asset_server.load("fonts/Retroville NC.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(Justify::Left),
                        ScoreUi,
                    ));
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("GAME OVER"),
                        TextFont {
                            font: asset_server.load("fonts/Retroville NC.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(Justify::Left),
                    ));
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

    *score_visibility_query.single_mut().unwrap().1 = if *play_state != PlayState::Ready {
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
                    game_state.set(GameState::WinScreen); //TODO: Fix this
                }
                PlayButtonAction::Pause => play_state.set(PlayState::Paused),
            }
        }
    }
}

pub(super) fn update_score_text(
    query: Query<Entity, With<ScoreUi>>,
    mut writer: TextUiWriter,
    points: Res<Points>,
) {
    let entity = query.single().unwrap();
    *writer.text(entity, 0) = format!("SCORE: {}", points.0);
}

pub(super) fn go_back_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    play_state.set(PlayState::Ready);
    game_state.set(GameState::Menu);
}
