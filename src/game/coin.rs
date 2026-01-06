use crate::game::anim::{AnimationIndices, AnimationTimer};

use super::{GameDynamicEntity, GameEntity};
use bevy::prelude::*;

//const COIN_ANIMATION_SPEED: f32 = 10.0;
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
        UVec2::new(32, 32),
        3,
        1,
        None,
        None,
    ));

    let animation_indices = AnimationIndices {
        first: 0,
        last: 2,
        ..default()
    };

    // Spawn coin
    commands.spawn((
        Coin,
        GameEntity,
        GameDynamicEntity,
        Sprite::from_atlas_image(
            coin_texture,
            TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
        ),
        Transform::from_xyz(position.x, position.y, 20.0),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub(super) fn animate_coins(
    mut coins: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<Coin>>,
    time: Res<Time>,
) {
    for (indices, mut timer, mut sprite) in &mut coins {
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
    }
}
