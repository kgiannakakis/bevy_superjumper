use super::GameEntity;
use bevy::prelude::*;

const BOB_ANIMATION_SPEED: f32 = 10.0;

#[derive(Component, Default)]
pub struct Bob {
    velocity: f32,
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
            transform: Transform::from_xyz(0.0, 0.0, 20.0),
            ..Default::default()
        },
    ));
}

pub(super) fn animate_bob(
    mut bob_query: Query<&mut TextureAtlasSprite, With<Bob>>,
    time: Res<Time>,
) {
    for mut bob in &mut bob_query {
        bob.index = 2 + ((time.elapsed_seconds() * BOB_ANIMATION_SPEED) as usize % 2);
    }
}
