use bevy::reflect::Reflect;
use leafwing_input_manager::prelude::Actionlike;

use cricket_pong_game::actions::{BatterAction, FielderAction};

#[derive(Actionlike, Reflect, Clone, Copy, Debug)]
pub enum BatterControl {
    SwingCW,
    SwingCCW,
    MoveCW,
    MoveCCW,
}

impl From<BatterControl> for BatterAction {
    fn from(control: BatterControl) -> Self {
        match control {
            BatterControl::SwingCW => BatterAction::SwingCW,
            BatterControl::SwingCCW => BatterAction::SwingCCW,
            BatterControl::MoveCW => BatterAction::MoveCW,
            BatterControl::MoveCCW => BatterAction::MoveCCW,
        }
    }
}

#[derive(Actionlike, Reflect, Clone, Copy, Debug)]
pub enum FielderControl {
    Bowl,
    MoveInfieldCW,
    MoveInfieldCCW,
    MoveOutfieldCW,
    MoveOutfieldCCW,
}

impl From<FielderControl> for FielderAction {
    fn from(control: FielderControl) -> Self {
        match control {
            FielderControl::Bowl => FielderAction::Bowl,
            FielderControl::MoveInfieldCW => FielderAction::MoveInfieldCW,
            FielderControl::MoveInfieldCCW => FielderAction::MoveInfieldCCW,
            FielderControl::MoveOutfieldCW => FielderAction::MoveOutfieldCW,
            FielderControl::MoveOutfieldCCW => FielderAction::MoveOutfieldCCW,
        }
    }
}
