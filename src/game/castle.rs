use super::GameEntity;
use bevy::prelude::*;

pub const CASTLE_SIZE: Vec2 = Vec2::new(64.0, 64.0);

#[derive(Component, Default)]
pub struct Castle;

pub(super) fn spawn_castle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) {
    let castle_texture = asset_server.load("sprites/castle.png");

    // Spawn spring
    commands.spawn((
        Castle,
        GameEntity,
        SpriteBundle {
            texture: castle_texture,
            transform: Transform::from_xyz(position.x, position.y, 20.0),
            ..Default::default()
        },
    ));
}
