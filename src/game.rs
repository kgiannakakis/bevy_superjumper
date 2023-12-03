use crate::{cleanup, AudioHandles, GameState, SoundDisabled};
use bevy::prelude::*;
use rand::Rng;

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
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

const WORLD_WIDTH: f32 = 10.0 * 32.0;
const WORLD_HEIGHT: f32 = 2.0 * 32.0 * 20.0;

fn setup_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut y: f32 = platform::PLATFORM_HEIGHT / 2.0;
    let max_jump_height: f32 =
        bob::BOB_JUMP_VELOCITY * bob::BOB_JUMP_VELOCITY / (2.0 * -bob::GRAVITY_Y);
    let mut rng = rand::thread_rng();
    while y < WORLD_HEIGHT - WORLD_WIDTH / 2.0 {
        let moving_platform = rng.gen_range(0.0..1.0) > 0.8;
        let x = rng.gen_range(0.0..1.0) * (WORLD_WIDTH - platform::PLATFORM_WIDTH)
            + platform::PLATFORM_WIDTH / 2.0;

        platform::spawn_platform(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            if moving_platform {
                platform::PlatformType::Moving
            } else {
                platform::PlatformType::Static
            },
            Vec2::new(x - 160.0, y - 240.0),
        );

        y += max_jump_height - 0.5 * 32.0;
        y -= rng.gen_range(0.0..1.0) * (max_jump_height / 3.0);
    }

    bob::setup_bob(&mut commands, &asset_server, &mut texture_atlases);

    coin::spawn_coin(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec2::new(0.0, 120.0),
    );

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
