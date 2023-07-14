use std::fmt::Display;

use bevy_ecs::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Identity {
    One,
    Two,
}

impl Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Identity {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Identity::One),
            2 => Ok(Identity::Two),
            _ => Err("Player identity should be 1 or 2".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Objective {
    Fielding,
    Batting,
}

impl Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component)]
pub struct Player {
    pub score: u16,
    pub objective: Objective,
    pub id: Identity,
}

impl Player {
    pub fn new(objective: Objective, id: Identity) -> Self {
        Player {
            score: 0,
            objective,
            id,
        }
    }
}
