use std::fmt::Display;

use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate, Serde};

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
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

#[derive(Clone, Copy, Debug, PartialEq, Component, Serde)]
pub enum Position {
    Fielder,
    Batter,
}

impl std::ops::Not for Position {
    type Output = Position;

    fn not(self) -> Self::Output {
        match self {
            Position::Fielder => Position::Batter,
            Position::Batter => Position::Fielder,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component, Replicate)]
pub struct Score {
    pub value: Property<u16>,
}

impl Default for Score {
    fn default() -> Self {
        Score::new_complete(0)
    }
}

#[derive(Component, Replicate)]
pub struct PlayerOne;

#[derive(Component, Replicate)]
pub struct PlayerTwo;
