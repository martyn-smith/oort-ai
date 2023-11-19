// Tutorial: Rotation
// Destroy the asteroid. The target is in a random
// location given by the "target()" function.
//
// Still easy, though note we are wasting bullets.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        let theta = angle_diff(heading(), (target() - position()).angle());
        // returns the direction your ship needs to turn to face the target.
        turn(theta);
        fire(0);
    }
}
