use super::{GameEntity, PlayState};
use crate::{
    Background,
    game::anim::{AnimationIndices, AnimationTimer},
};
use bevy::prelude::*;

//const BOB_ANIMATION_SPEED: f32 = 10.0;
pub const BOB_JUMP_VELOCITY: f32 = 400.0; // 11
const BOB_MOVE_VELOCITY: f32 = 500.0; // 20
const ACCELERATION_X: f32 = 0.5;
pub const GRAVITY_Y: f32 = -480.0; // -12
pub const BOB_SIZE: Vec2 = Vec2::new(32.0, 32.0);

#[derive(Component, Default)]
pub struct Bob {
    pub velocity: Vec2,
}

pub(super) fn setup_bob(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the bob's sprite sheet and create a texture atlas from it
    let bob_texture = asset_server.load("sprites/bob.png");
    let layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        5,
        1,
        None,
        None,
    ));

    let animation_indices = AnimationIndices {
        first: 0,
        last: 3,
        death: 4,
    };

    // Spawn bob
    commands.spawn((
        Bob::default(),
        GameEntity,
        Sprite::from_atlas_image(
            bob_texture,
            TextureAtlas {
                layout: layout_handle,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, -240.0 + 32.0, 20.0),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub(super) fn animate_bob(
    mut bob_query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut Transform,
            &mut Sprite,
            &Bob,
        ),
        With<Bob>,
    >,
    time: Res<Time>,
) {
    for (indices, mut timer, mut transform, mut sprite, bob) in &mut bob_query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }

        if bob.velocity.x < 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            transform.rotation = Quat::default();
        }
    }
}

pub(super) fn update_bob(
    mut bob_query: Query<(&mut Transform, &mut Bob), With<Bob>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Bob>, Without<Background>)>,
    mut bg_query: Query<&mut Transform, (With<Background>, Without<Bob>, Without<Camera>)>,
    play_state: Res<State<PlayState>>,
    time: Res<Time>,
) {
    let (mut transform, mut bob) = bob_query.single_mut().unwrap();
    let mut camera = camera_query.single_mut().unwrap();

    bob.velocity.y += GRAVITY_Y * time.delta_secs();

    if *play_state == PlayState::GameOver {
        if bob.velocity.y >= 0.0 {
            bob.velocity.y = 0.0;
        }
        if transform.translation.y > camera.translation.y - 240.0 {
            transform.translation.y += bob.velocity.y * time.delta_secs();
        }
    } else {
        transform.translation.x += bob.velocity.x * time.delta_secs();
        transform.translation.y += bob.velocity.y * time.delta_secs();

        if transform.translation.y < -240.0 + 16.0 {
            bob.velocity.y = BOB_JUMP_VELOCITY;
        }

        if transform.translation.x < -160.0 {
            transform.translation.x += 320.0;
        }
        if transform.translation.x > 160.0 {
            transform.translation.x -= 320.0;
        }

        if transform.translation.y > camera.translation.y {
            camera.translation.y = transform.translation.y;
            bg_query.single_mut().unwrap().translation.y = transform.translation.y;
        }
    }
}

pub(super) fn move_bob(
    mut bob: Query<&mut Bob, With<Bob>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut bob in &mut bob {
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            bob.velocity.x = ACCELERATION_X * BOB_MOVE_VELOCITY;
        } else if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyA)
        {
            bob.velocity.x = -ACCELERATION_X * BOB_MOVE_VELOCITY;
        } else {
            bob.velocity.x = 0.0;
        }
    }
}

pub(super) fn animate_bob_death(mut bob_query: Query<(&AnimationIndices, &mut Sprite), With<Bob>>) {
    for (indices, mut sprite) in &mut bob_query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indices.death;
        }
    }
}

pub(super) fn check_bob_has_fallen(
    bob_query: Query<&Transform, With<Bob>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Bob>)>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    let bob_transform = bob_query.single();
    let camera = camera_query.single().unwrap();

    if bob_transform.unwrap().translation.y <= camera.translation.y - 240.0
        && camera.translation.y > 0.0
    {
        play_state.set(PlayState::GameOver);
    }
}
