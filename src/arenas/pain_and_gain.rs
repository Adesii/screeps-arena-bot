use log::warn;
use screeps_arena::{constants::prototypes, game, prelude::*};

pub(super) fn tick() {
    let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
    warn!("creeps {}", creeps.len());

    let enemy_spawn = game::utils::get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)
        .into_iter()
        .find(|spawn| spawn.my() == Some(false));
    if let Some(target) = enemy_spawn {
        for creep in creeps {
            if creep.my() {
                let _ = creep.move_to(target.as_ref(), None);
                let _ = creep.attack(&target);
            }
        }
    }
}
