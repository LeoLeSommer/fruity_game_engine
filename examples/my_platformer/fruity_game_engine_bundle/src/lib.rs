pub use fruity_ecs::*;
pub use fruity_game_engine::*;
pub use fruity_hierarchy::*;
use napi_derive::napi;

pub extern crate fruity_ecs;
pub extern crate fruity_game_engine;
pub extern crate fruity_hierarchy;

//#[napi]
fn sum(a: u32, b: u32) -> u32 {
    a + b
}
