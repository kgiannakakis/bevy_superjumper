use rand::Rng;

use super::{bob, platform};

const WORLD_WIDTH: f32 = 10.0 * 32.0;
const WORLD_HEIGHT: f32 = 2.0 * 32.0 * 20.0;

pub enum GameObjectType {
    Platform(bool),
    Squirrel,
    Coin,
    Spring,
}
pub struct GameObject {
    object_type: GameObjectType,
    x: f32,
    y: f32,
}

pub fn generate_level() -> Vec<GameObject> {
    let mut objects: Vec<GameObject> = Vec::new();
    let mut y: f32 = platform::PLATFORM_HEIGHT / 2.0;
    let max_jump_height: f32 =
        bob::BOB_JUMP_VELOCITY * bob::BOB_JUMP_VELOCITY / (2.0 * -bob::GRAVITY_Y);
    let mut rng = rand::thread_rng();
    while y < WORLD_HEIGHT - WORLD_WIDTH / 2.0 {
        let moving = rng.gen_range(0.0..1.0) > 0.8;
        let x = rng.gen_range(0.0..1.0) * (WORLD_WIDTH - platform::PLATFORM_WIDTH)
            + platform::PLATFORM_WIDTH / 2.0;

        objects.push(GameObject {
            object_type: GameObjectType::Platform(moving),
            x,
            y,
        });

        y += max_jump_height - 0.5 * 32.0;
        y -= rng.gen_range(0.0..1.0) * (max_jump_height / 3.0);
    }

    objects
}
