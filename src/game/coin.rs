use super::{GameDynamicEntity, GameEntity};
use bevy::prelude::*;

const COIN_ANIMATION_SPEED: f32 = 10.0;
pub const COIN_HEIGHT: f32 = 0.8 * 32.0;
pub const COIN_SIZE: Vec2 = Vec2::new(0.5 * 32.0, 0.5 * 32.0);
pub const COIN_SCORE: u32 = 10;

#[derive(Component, Default)]
pub struct Coin;

pub(super) fn spawn_coin(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    position: Vec2,
) {
    // Load the coin's sprite sheet and create a texture atlas from it
    let coin_texture = asset_server.load("sprites/coin.png");
    let layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        3,
        1,
        None,
        None,
    ));

    // Spawn coin
    commands.spawn((
        Coin,
        GameEntity,
        GameDynamicEntity,
        SpriteSheetBundle {
            texture: coin_texture,
            atlas: TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
            transform: Transform::from_xyz(position.x, position.y, 20.0),
            ..Default::default()
        },
    ));
}

pub(super) fn animate_coins(mut coins: Query<&mut TextureAtlas, With<Coin>>, time: Res<Time>) {
    for mut coin in &mut coins {
        coin.index = (time.elapsed_seconds() * COIN_ANIMATION_SPEED) as usize % 3;
    }
}
