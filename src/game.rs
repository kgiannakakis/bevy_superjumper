#![allow(clippy::type_complexity)]
use rand::Rng;

use crate::{
    cleanup, click_sound,
    help::has_user_input,
    highscores::{check_and_update_highscores, HighScores},
    play_sound, AudioHandles, Background, GameState, SoundEnabled,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

use bob::Bob;
use castle::Castle;
use coin::Coin;
use platform::Platform;
use spring::Spring;
use squirrel::Squirrel;

use self::level::GameObject;

mod bob;
mod castle;
mod coin;
mod level;
mod platform;
mod spring;
mod squirrel;

#[derive(Component)]
struct GameEntity;

#[derive(Component)]
struct GameDynamicEntity;

#[derive(Component)]
struct MovingObject {
    width: f32,
    velocity_x: f32,
    dir: f32,
}

#[derive(Component)]
struct GameUi;

#[derive(Component)]
struct GameButtonUi;

#[derive(Component)]
struct ScoreUi;

#[derive(Component)]
struct GameOverUi;

#[derive(Resource, Default)]
pub struct Points(u32);

#[derive(Resource, Default)]
pub struct GameObjects(Vec<GameObject>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayState {
    #[default]
    Ready,
    Running,
    Paused,
    GameOver,
}

#[derive(Component)]
enum PlayButtonAction {
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

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayState>()
            .init_resource::<Points>()
            .init_resource::<GameObjects>()
            .add_systems(OnEnter(GameState::Playing), setup_play)
            .add_systems(
                OnExit(GameState::Playing),
                (click_sound, cleanup::<GameEntity>, reset_play),
            )
            .add_systems(
                Update,
                (
                    ui_action,
                    update_buttons_visibility.run_if(state_changed::<PlayState>()),
                    click_sound.run_if(state_changed::<PlayState>()),
                    spawn_objects,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    coin_sound.run_if(
                        resource_changed::<Points>().and_then(not(resource_added::<Points>())),
                    ),
                    update_score_text.run_if(resource_changed::<Points>()),
                    bob::animate_bob,
                    bob::update_bob,
                    bob::move_bob,
                    bob::check_bob_has_fallen,
                    coin::animate_coins,
                    squirrel::animate_squirrels,
                    platform::animate_platforms,
                    move_objects,
                    check_platform_collisions,
                    check_coin_collisions,
                    check_squirrel_collisions,
                    check_spring_collisions,
                    check_castle_collisions,
                    cleanup_objects,
                )
                    .run_if(in_state(GameState::Playing).and_then(in_state(PlayState::Running))),
            )
            .add_systems(
                Update,
                (go_back_to_menu.run_if(has_user_input), bob::update_bob)
                    .run_if(in_state(GameState::Playing).and_then(in_state(PlayState::GameOver))),
            )
            .add_systems(
                OnEnter(PlayState::GameOver),
                (bob::animate_bob_death, spawn_game_over_ui),
            );
    }
}

fn setup_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_objects: ResMut<GameObjects>,
) {
    game_objects.0 = level::generate_level();

    bob::setup_bob(&mut commands, &asset_server, &mut texture_atlases);

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
                            .with_text_alignment(TextAlignment::Center),
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
                        .with_text_alignment(TextAlignment::Left),
                        ScoreUi,
                    ));
                });
        });
}

fn spawn_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_objects: ResMut<GameObjects>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let max_y = camera_query.single().translation.y + 1.1 * 480.0;

    for object in &mut game_objects.0 {
        // Only spawn objects that are on screen and a 10% above
        if object.is_spawned || object.y > max_y {
            continue;
        }

        object.is_spawned = true;

        match object.object_type {
            level::GameObjectType::Platform(moving) => {
                platform::spawn_platform(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    moving,
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
            level::GameObjectType::Squirrel => {
                squirrel::spawn_squirrel(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
            level::GameObjectType::Coin => {
                coin::spawn_coin(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
            level::GameObjectType::Spring => {
                spring::spawn_spring(
                    &mut commands,
                    &asset_server,
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
            level::GameObjectType::Castle => {
                castle::spawn_castle(
                    &mut commands,
                    &asset_server,
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
        }
    }
}

fn cleanup_objects(
    mut commands: Commands,
    mut game_objects: ResMut<GameObjects>,
    mut dynamic_objects: Query<(Entity, &Transform), With<GameDynamicEntity>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let min_y = camera_query.single().translation.y - 1.2 * 240.0;

    game_objects.0.retain(|o| !o.is_spawned);

    for (entity, transform) in &mut dynamic_objects {
        // Despawn objects that are below screen's bottom
        if transform.translation.y < min_y {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_game_over_ui(
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
                        .with_text_alignment(TextAlignment::Left),
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
                    .with_text_alignment(TextAlignment::Left),));
                });
        });
}

fn check_platform_collisions(
    mut bob_query: Query<(&Transform, &mut Bob), With<Bob>>,
    mut platforms_query: Query<(&Transform, &mut Platform), With<Platform>>,
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_enabled: Res<SoundEnabled>,
) {
    let (&bob_transform, mut bob) = bob_query.single_mut();
    if bob.velocity.y > 0.0 {
        return;
    }

    for (&platform_transform, mut platform) in &mut platforms_query {
        if collide(
            bob_transform.translation,
            bob::BOB_SIZE,
            platform_transform.translation,
            platform::PLATFORM_SIZE,
        )
        .is_some()
        {
            bob.velocity.y = bob::BOB_JUMP_VELOCITY;

            play_sound(audio_handles.jump.clone(), &mut commands, &sound_enabled);

            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) > 0.5 {
                platform.state = platform::PlatformState::Pulverizing;
            }
            return;
        }
    }
}

fn check_spring_collisions(
    mut bob_query: Query<(&Transform, &mut Bob), With<Bob>>,
    springs_query: Query<&Transform, With<Spring>>,
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_enabled: Res<SoundEnabled>,
) {
    let (&bob_transform, mut bob) = bob_query.single_mut();

    if bob.velocity.y > 0.0 {
        return;
    }

    for &spring_transform in &springs_query {
        if collide(
            bob_transform.translation,
            bob::BOB_SIZE,
            spring_transform.translation,
            spring::SPRING_SIZE,
        )
        .is_some()
        {
            bob.velocity.y = bob::BOB_JUMP_VELOCITY * 1.5;
            play_sound(
                audio_handles.highjump.clone(),
                &mut commands,
                &sound_enabled,
            );
            return;
        }
    }
}

fn check_coin_collisions(
    bob_query: Query<&Transform, With<Bob>>,
    mut coins_query: Query<(Entity, &Transform), With<Coin>>,
    mut points: ResMut<Points>,
    mut commands: Commands,
) {
    let bob_transform = bob_query.single();
    for (entity, &coin_transform) in &mut coins_query {
        if collide(
            bob_transform.translation,
            bob::BOB_SIZE,
            coin_transform.translation,
            coin::COIN_SIZE,
        )
        .is_some()
        {
            points.0 += coin::COIN_SCORE;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_squirrel_collisions(
    bob_query: Query<&Transform, With<Bob>>,
    mut squirrels_query: Query<&Transform, With<Squirrel>>,
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_enabled: Res<SoundEnabled>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    let bob_transform = bob_query.single();
    for &squirrel_transform in &mut squirrels_query {
        if collide(
            bob_transform.translation,
            bob::BOB_SIZE,
            squirrel_transform.translation,
            squirrel::SQUIRREL_SIZE,
        )
        .is_some()
        {
            play_sound(audio_handles.hit.clone(), &mut commands, &sound_enabled);
            play_state.set(PlayState::GameOver);
            return;
        }
    }
}

fn check_castle_collisions(
    bob_query: Query<&Transform, With<Bob>>,
    castles_query: Query<&Transform, With<Castle>>,
    mut game_state: ResMut<NextState<GameState>>,
    points: Res<Points>,
    mut high_scores: ResMut<HighScores>,
) {
    let bob_transform = bob_query.single();
    for castle_transform in &castles_query {
        if collide(
            bob_transform.translation,
            bob::BOB_SIZE,
            castle_transform.translation,
            castle::CASTLE_SIZE,
        )
        .is_some()
        {
            check_and_update_highscores(&mut high_scores, points.0);
            game_state.set(GameState::WinScreen);
        }
    }
}

fn move_objects(
    mut objects_query: Query<(&mut MovingObject, &mut Transform), With<MovingObject>>,
    time: Res<Time>,
) {
    for (mut obj, mut transform) in &mut objects_query {
        transform.translation.x += obj.velocity_x * obj.dir * time.delta_seconds();

        if transform.translation.x + obj.width / 2.0 > 160.0 {
            obj.dir = -1.0;
        } else if transform.translation.x - obj.width / 2.0 < -160.0 {
            obj.dir = 1.0;
        }
    }
}

fn coin_sound(
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_enabled: Res<SoundEnabled>,
) {
    if sound_enabled.0 {
        commands.spawn(AudioBundle {
            source: audio_handles.coin.clone(),
            ..default()
        });
    }
}

fn ui_action(
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

fn update_buttons_visibility(
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

fn update_score_text(mut query: Query<&mut Text, With<ScoreUi>>, points: Res<Points>) {
    for mut text in &mut query {
        text.sections[0].value = format!("SCORE: {}", points.0);
    }
}

fn reset_play(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Background>)>,
    mut bg_query: Query<&mut Transform, (With<Background>, Without<Camera>)>,
    mut points: ResMut<Points>,
) {
    camera_query.single_mut().translation.y = 0.0;
    bg_query.single_mut().translation.y = 0.0;
    points.0 = 0;
}

fn go_back_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    play_state.set(PlayState::Ready);
    game_state.set(GameState::Menu);
}
