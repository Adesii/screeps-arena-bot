use log::warn;
use screeps_arena::{constants::prototypes, game};

pub(super) fn tick() {
    let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
    warn!("creeps {}", creeps.len());
}
