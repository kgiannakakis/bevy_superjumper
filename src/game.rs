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
mod game_ui;
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

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayState>()
            .init_resource::<Points>()
            .init_resource::<GameObjects>()
            .add_systems(OnEnter(GameState::Playing), (setup_play, game_ui::setup_ui))
            .add_systems(
                OnExit(GameState::Playing),
                (click_sound, cleanup::<GameEntity>, reset_play),
            )
            .add_systems(
                Update,
                (
                    game_ui::ui_action,
                    game_ui::update_buttons_visibility.run_if(state_changed::<PlayState>()),
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
                    game_ui::update_score_text.run_if(resource_changed::<Points>()),
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
                (
                    game_ui::go_back_to_menu.run_if(has_user_input),
                    bob::update_bob,
                )
                    .run_if(in_state(GameState::Playing).and_then(in_state(PlayState::GameOver))),
            )
            .add_systems(
                OnEnter(PlayState::GameOver),
                (bob::animate_bob_death, game_ui::spawn_game_over_ui),
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

fn check_platform_collisions(
    mut bob_query: Query<(&Transform, &mut Bob), With<Bob>>,
    mut platforms_query: Query<(&Transform, &mut Platform), With<Platform>>,
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_enabled: Res<SoundEnabled>,
    time: Res<Time>,
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
                platform.state = platform::PlatformState::Pulverizing(time.elapsed_seconds());
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
    mut play_state: ResMut<NextState<PlayState>>,
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
            play_state.set(PlayState::Ready);
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

fn reset_play(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Background>)>,
    mut bg_query: Query<&mut Transform, (With<Background>, Without<Camera>)>,
    mut points: ResMut<Points>,
) {
    camera_query.single_mut().translation.y = 0.0;
    bg_query.single_mut().translation.y = 0.0;
    points.0 = 0;
}
