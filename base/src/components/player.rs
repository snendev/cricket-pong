use std::fmt::Display;

use bevy_core::Name;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::Reflect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Identity {
    One,
    Two,
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

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum Position {
    #[default]
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

impl Position {
    pub fn is_fielder(&self) -> bool {
        *self == Position::Fielder
    }

    pub fn is_batter(&self) -> bool {
        *self == Position::Batter
    }

    pub fn switch(&mut self) {
        *self = !*self;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerOne;

impl PlayerOne {
    pub fn name() -> Name {
        Name::new("Player One")
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerTwo;

impl PlayerTwo {
    pub fn name() -> Name {
        Name::new("Player Two")
    }
}
