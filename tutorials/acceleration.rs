// Tutorial: Acceleration
// Fly through the target circle.
//
// Similar to #1 (guns), not much to say.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        // Hint: uncomment me
        accelerate(vec2(100.0, 0.0));
    }
}
