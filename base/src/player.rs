use bevy_ecs::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Objective {
    Fielding,
    Batting,
}

#[derive(Component)]
pub struct Player {
    pub score: u16,
    pub objective: Objective,
}

impl Player {
    pub fn new(objective: Objective) -> Self {
        Player {
            score: 0,
            objective,
        }
    }
}
