use crate::{cleanup, click_sound, AudioHandles, GameState, SoundDisabled};
use bevy::{prelude::*, sprite::collide_aabb::collide};

use bob::Bob;
use platform::Platform;

mod bob;
mod coin;
mod level;
mod platform;

#[derive(Component)]
struct GameEntity;

#[derive(Resource, Default)]
pub struct Points(usize);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayState {
    #[default]
    Ready,
    // Running,
    // Paused,
    // LevelEnd,
    // GameOver,
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
            .add_systems(OnEnter(GameState::Playing), setup_play)
            .add_systems(
                OnExit(GameState::Playing),
                (click_sound, cleanup::<GameEntity>),
            )
            .add_systems(
                Update,
                (
                    coin_sound.run_if(
                        resource_changed::<Points>().and_then(not(resource_added::<Points>())),
                    ),
                    bob::animate_bob,
                    bob::update_bob,
                    bob::move_bob,
                    coin::animate_coins,
                    platform::animate_platforms,
                    check_platform_collisions,
                    ui_action,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let objects = level::generate_level();

    for object in &objects {
        match object.object_type {
            level::GameObjectType::Platform(moving) => {
                platform::spawn_platform(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    if moving {
                        platform::PlatformType::Moving
                    } else {
                        platform::PlatformType::Static
                    },
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
            level::GameObjectType::Squirrel => todo!(),
            level::GameObjectType::Coin => {
                coin::spawn_coin(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    Vec2::new(object.x - 160.0, object.y - 240.0),
                );
            }
            level::GameObjectType::Spring => todo!(),
        }
    }

    bob::setup_bob(&mut commands, &asset_server, &mut texture_atlases);

    // Spawn the score UI
    commands
        .spawn((
            GameEntity,
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
                            visibility: visibility,
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
                        ..default()
                    },
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
        });
}

fn check_platform_collisions(
    mut bob_query: Query<(&Transform, &mut Bob), With<Bob>>,
    mut platforms_query: Query<(&Transform, &mut Platform), With<Platform>>,
) {
    for (&bob_transform, mut bob) in &mut bob_query {
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
                platform.state = platform::PlatformState::Pulverizing;
                return;
            }
        }
    }
}

fn coin_sound(
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_disabled: Res<SoundDisabled>,
) {
    if !sound_disabled.0 {
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
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PlayButtonAction::Play => {}
                PlayButtonAction::Resume => {}
                PlayButtonAction::Quit => game_state.set(GameState::Menu),
                PlayButtonAction::Pause => game_state.set(GameState::Menu),
            }
        }
    }
}
