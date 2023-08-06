use bevy_ecs::prelude::{Entity, Resource};

use naia_bevy_shared::Serde;

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum BatterAction {
    SwingCW,
    SwingCCW,
    MoveCW,
    MoveCCW,
}

impl BatterAction {
    pub fn rotation_direction(&self) -> f32 {
        match self {
            BatterAction::MoveCW | BatterAction::SwingCW => -1.,
            BatterAction::MoveCCW | BatterAction::SwingCCW => 1.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub enum FielderAction {
    Bowl,
    MoveInfieldCW,
    MoveInfieldCCW,
    MoveOutfieldCW,
    MoveOutfieldCCW,
}

impl FielderAction {
    pub fn rotation_direction(&self) -> Option<f32> {
        match self {
            FielderAction::MoveInfieldCW | FielderAction::MoveOutfieldCW => Some(-1.),
            FielderAction::MoveInfieldCCW | FielderAction::MoveOutfieldCCW => Some(1.),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serde)]
pub enum Action {
    Fielder(FielderAction),
    Batter(BatterAction),
}

#[derive(Clone, Debug, Default, Resource)]
pub struct Actions(pub Vec<(Entity, Action)>);
