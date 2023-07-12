use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Ball;

impl Ball {
    pub const RADIUS: f32 = 8.;
}
