use std::fmt::Display;

use bevy_core::Name;
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
pub enum PositionKind {
    Fielder,
    Batter,
}

impl std::ops::Not for PositionKind {
    type Output = PositionKind;

    fn not(self) -> Self::Output {
        match self {
            PositionKind::Fielder => PositionKind::Batter,
            PositionKind::Batter => PositionKind::Fielder,
        }
    }
}

#[derive(Component, Replicate)]
pub struct Position {
    kind: Property<PositionKind>,
}

impl Position {
    pub fn fielder() -> Self {
        Position::new_complete(PositionKind::Fielder)
    }

    pub fn batter() -> Self {
        Position::new_complete(PositionKind::Batter)
    }

    pub fn inner(&self) -> PositionKind {
        *self.kind
    }

    pub fn is_kind(&self, kind: PositionKind) -> bool {
        *self.kind == kind
    }

    pub fn is_fielder(&self) -> bool {
        *self.kind == PositionKind::Fielder
    }

    pub fn is_batter(&self) -> bool {
        *self.kind == PositionKind::Batter
    }

    pub fn switch(&mut self) {
        *self.kind = !*self.kind;
    }
}

impl Display for PositionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Component, Replicate)]
pub struct PlayerOne;

impl PlayerOne {
    pub fn name() -> Name {
        Name::new("Player One")
    }
}

#[derive(Component, Replicate)]
pub struct PlayerTwo;

impl PlayerTwo {
    pub fn name() -> Name {
        Name::new("Player Two")
    }
}
