// Tutorial: Acceleration 2
// Fly through the target circle. The target is in a random
// location given by the "target" function.
//
// ...
// Note, the ship does not need to accelerate in the direction it is pointing.
// turn() could also be used but to no gain.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        let v = target() - position();
        accelerate(v);
    }
}

