use super::GameEntity;
use bevy::prelude::*;

pub const SPRING_HEIGHT: f32 = 0.3 * 32.0;
pub const SPRING_SIZE: Vec2 = Vec2::new(32.0, 32.0);

#[derive(Component, Default)]
pub struct Spring;

pub(super) fn spawn_spring(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) {
    let spring_texture = asset_server.load("sprites/spring.png");

    // Spawn spring
    commands.spawn((
        Spring,
        GameEntity,
        SpriteBundle {
            texture: spring_texture,
            transform: Transform::from_xyz(position.x, position.y, 20.0),
            ..Default::default()
        },
    ));
}
