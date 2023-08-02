use bevy::{
    prelude::{Component, Entity, ReflectComponent},
    reflect::Reflect,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PredictionOf(pub Entity);

impl Default for PredictionOf {
    fn default() -> Self {
        PredictionOf(Entity::PLACEHOLDER)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SourceOf(pub Entity);

impl Default for SourceOf {
    fn default() -> Self {
        SourceOf(Entity::PLACEHOLDER)
    }
}
