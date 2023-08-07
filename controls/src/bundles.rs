use bevy::prelude::{Bundle, KeyCode};

use leafwing_input_manager::prelude::{ActionState, InputManagerBundle, InputMap};

use crate::{BatterControl, FielderControl};

#[derive(Bundle)]
pub struct BatterControllerBundle {
    input_manager: InputManagerBundle<BatterControl>,
}

impl BatterControllerBundle {
    pub fn new() -> Self {
        let input_map = InputMap::new([
            (KeyCode::Q, BatterControl::SwingCCW),
            (KeyCode::W, BatterControl::SwingCW),
            (KeyCode::A, BatterControl::MoveCCW),
            (KeyCode::S, BatterControl::MoveCW),
        ])
        .build();
        BatterControllerBundle {
            input_manager: InputManagerBundle::<BatterControl> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }
}

impl Default for BatterControllerBundle {
    fn default() -> Self {
        BatterControllerBundle::new()
    }
}

#[derive(Bundle)]
pub struct BatterControllerBundle2 {
    input_manager: InputManagerBundle<BatterControl>,
}

impl BatterControllerBundle2 {
    pub fn new() -> Self {
        let input_map = InputMap::new([
            (KeyCode::U, BatterControl::SwingCCW),
            (KeyCode::I, BatterControl::SwingCW),
            (KeyCode::J, BatterControl::MoveCCW),
            (KeyCode::K, BatterControl::MoveCW),
        ])
        .build();
        BatterControllerBundle2 {
            input_manager: InputManagerBundle::<BatterControl> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }
}

impl Default for BatterControllerBundle2 {
    fn default() -> Self {
        BatterControllerBundle2::new()
    }
}

#[derive(Bundle)]
pub struct FielderControllerBundle {
    input_manager: InputManagerBundle<FielderControl>,
}

impl FielderControllerBundle {
    pub fn new() -> Self {
        let input_map = InputMap::new([
            (KeyCode::Space, FielderControl::Bowl),
            (KeyCode::Q, FielderControl::MoveOutfieldCCW),
            (KeyCode::W, FielderControl::MoveOutfieldCW),
            (KeyCode::A, FielderControl::MoveInfieldCCW),
            (KeyCode::S, FielderControl::MoveInfieldCW),
        ])
        .build();
        FielderControllerBundle {
            input_manager: InputManagerBundle::<FielderControl> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }
}

impl Default for FielderControllerBundle {
    fn default() -> Self {
        FielderControllerBundle::new()
    }
}

#[derive(Bundle)]
pub struct FielderControllerBundle2 {
    input_manager: InputManagerBundle<FielderControl>,
}

impl FielderControllerBundle2 {
    pub fn new() -> Self {
        let input_map = InputMap::new([
            (KeyCode::ShiftRight, FielderControl::Bowl),
            (KeyCode::U, FielderControl::MoveOutfieldCCW),
            (KeyCode::I, FielderControl::MoveOutfieldCW),
            (KeyCode::J, FielderControl::MoveInfieldCCW),
            (KeyCode::K, FielderControl::MoveInfieldCW),
        ])
        .build();
        FielderControllerBundle2 {
            input_manager: InputManagerBundle::<FielderControl> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }
}

impl Default for FielderControllerBundle2 {
    fn default() -> Self {
        FielderControllerBundle2::new()
    }
}
