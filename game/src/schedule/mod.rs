use bevy_ecs::{
    prelude::{In, IntoSystemConfigs, IntoSystemSetConfigs, Schedule, World},
    schedule::ScheduleLabel,
};

use cricket_pong_base::actions::Actions;

use crate::systems::{scoring, tick};

mod physics;
pub(crate) use physics::InstanceFilter;

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct CoreTickSchedule;

#[derive(Debug)]
struct InSchedule;

pub(crate) fn build_core_tick_schedule() -> (CoreTickSchedule, Schedule) {
    let schedule = Schedule::new();
    let mut schedule = physics::build_physics_schedule(schedule);
    schedule
        .configure_sets((tick::ActionsSet, physics::PhysicsSet, scoring::ScoringSet).chain())
        .add_systems((
            (tick::track_bowler_transform, tick::consume_actions).in_set(tick::ActionsSet),
            scoring::register_goals.in_set(scoring::ScoringSet),
        ));
    (CoreTickSchedule, schedule)
}

pub(crate) fn run_core_game_loop(In(ticks): In<Vec<(u16, Actions)>>, world: &mut World) {
    for (_tick, tick_actions) in ticks {
        let mut inputs = world.resource_mut::<Actions>();
        inputs.0 = tick_actions.0;
        world.run_schedule(CoreTickSchedule);
    }
}
