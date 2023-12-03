use super::GameEntity;
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
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    // Load the bob's sprite sheet and create a texture atlas from it
    let bob_texture = asset_server.load("sprites/bob.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        bob_texture,
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
            texture_atlas,
            transform: Transform::from_xyz(0.0, -240.0 + 32.0, 20.0),
            ..Default::default()
        },
    ));
}

pub(super) fn animate_bob(
    mut bob_query: Query<(&mut TextureAtlasSprite, &mut Transform, &Bob), With<Bob>>,
    time: Res<Time>,
) {
    for (mut bob_ta, mut transform, bob) in &mut bob_query {
        bob_ta.index = 2 + ((time.elapsed_seconds() * BOB_ANIMATION_SPEED) as usize % 2);

        if bob.velocity.x < 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            transform.rotation = Quat::default();
        }
    }
}

pub(super) fn update_bob(mut bob: Query<(&mut Transform, &mut Bob), With<Bob>>, time: Res<Time>) {
    for (mut transform, mut bob) in &mut bob {
        bob.velocity.y += GRAVITY_Y * time.delta_seconds();
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
    }
}

pub(super) fn move_bob(mut bob: Query<&mut Bob, With<Bob>>, keyboard_input: Res<Input<KeyCode>>) {
    for mut bob in &mut bob {
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            bob.velocity.x = ACCELERATION_X * BOB_MOVE_VELOCITY;
        } else if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            bob.velocity.x = -ACCELERATION_X * BOB_MOVE_VELOCITY;
        } else {
            bob.velocity.x = 0.0;
        }
    }
}
