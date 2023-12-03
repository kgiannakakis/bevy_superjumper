use crate::{cleanup, AudioHandles, GameState, SoundDisabled};
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

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayState>()
            .init_resource::<Points>()
            .add_systems(OnEnter(GameState::Playing), setup_play)
            .add_systems(OnExit(GameState::Playing), cleanup::<GameEntity>)
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
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|node| {
            node.spawn((TextBundle::from_section(
                "Super Jumper",
                TextStyle {
                    font: asset_server.load("fonts/Retroville NC.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),));
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
