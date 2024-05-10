use super::{GameEntity, PlayState};
use crate::Background;
use bevy::prelude::*;

const BOB_ANIMATION_SPEED: f32 = 10.0;
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
        Vec2::new(32.0, 32.0),
        5,
        1,
        None,
        None,
    ));

    // Spawn bob
    commands.spawn((
        Bob::default(),
        GameEntity,
        SpriteSheetBundle {
            texture: bob_texture,
            atlas: TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
            transform: Transform::from_xyz(0.0, -240.0 + 32.0, 20.0),
            ..Default::default()
        },
    ));
}

pub(super) fn animate_bob(
    mut bob_query: Query<(&mut TextureAtlas, &mut Transform, &Bob), With<Bob>>,
    time: Res<Time>,
) {
    let (mut bob_ta, mut transform, bob) = bob_query.single_mut();
    bob_ta.index = 2 + ((time.elapsed_seconds() * BOB_ANIMATION_SPEED) as usize % 2);

    if bob.velocity.x < 0.0 {
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    } else {
        transform.rotation = Quat::default();
    }
}

pub(super) fn update_bob(
    mut bob_query: Query<(&mut Transform, &mut Bob), With<Bob>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Bob>, Without<Background>)>,
    mut bg_query: Query<&mut Transform, (With<Background>, Without<Bob>, Without<Camera>)>,
    play_state: Res<State<PlayState>>,
    time: Res<Time>,
) {
    let (mut transform, mut bob) = bob_query.single_mut();
    let mut camera = camera_query.single_mut();

    bob.velocity.y += GRAVITY_Y * time.delta_seconds();

    if *play_state == PlayState::GameOver {
        if bob.velocity.y >= 0.0 {
            bob.velocity.y = 0.0;
        }
        if transform.translation.y > camera.translation.y - 240.0 {
            transform.translation.y += bob.velocity.y * time.delta_seconds();
        }
    } else {
        transform.translation.x += bob.velocity.x * time.delta_seconds();
        transform.translation.y += bob.velocity.y * time.delta_seconds();

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
            bg_query.single_mut().translation.y = transform.translation.y;
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

pub(super) fn animate_bob_death(mut bob_query: Query<&mut TextureAtlas, With<Bob>>) {
    let mut bob_ta = bob_query.single_mut();
    bob_ta.index = 4;
}

pub(super) fn check_bob_has_fallen(
    bob_query: Query<&Transform, With<Bob>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Bob>)>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    let bob_transform = bob_query.single();
    let camera = camera_query.single();

    if bob_transform.translation.y <= camera.translation.y - 240.0 && camera.translation.y > 0.0 {
        play_state.set(PlayState::GameOver);
    }
}
