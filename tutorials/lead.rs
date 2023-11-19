// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// The first really hard one.
//
// Approach:
//
// Spin up a Solver to generate a firing solution and PID controller to coerce the ship's
// heading towards that solution. In this scenario distance control is not needed.
// Also a bit cheeky, target position and velocity are available as library functions,
// so are acquired by the Solver rather than being supplied. Note the Solver is stateless.
//
// Manually tuned parameters:
//
//   - PID settings
//   - ANGULAR_TOLERANCE (fire if within this)
//   - number of loops for Solver.

use oort_api::prelude::*;
const BULLET_SPEED: f64 = 1000.0; // m/s
const ANGULAR_TOLERANCE: f64 = 0.005; //radians, tunable

mod PID {
    use oort_api::prelude::*;
    const P: f64 = 2.0; //a.u, P component of PID, tunable
    const I: f64 = 0.1; //a.i, I component of PID
    const D: f64 = 2.0; //a.u. D component of PID

    pub struct PID {
        integral: f64,
        derivative: f64
    }

    impl PID {
        pub fn new() -> Self {
            Self {
                integral: 0.,
                derivative: 0.
            }
        }

        pub fn pid(&mut self, err: f64) -> f64 {
            let total = err * P
                        + (err - self.derivative) * D
                        + self.integral * I;
            self.derivative = err;
            self.integral += err;
            if self.integral > TAU {
                self.integral -= TAU;
            } else if self.integral < -TAU {
                self.integral += TAU;
            }
            -total
        }
    }
}

mod Solver {
    use oort_api::prelude::*;
    use super::{BULLET_SPEED, Target};
    const LOOPS: u8 = 10;

    pub fn solve(base: Vec2, target: Target) -> f64 {
        let mut r = target.position - base;
        let mut r_prev = Vec2{x:0., y:0.};
        let v = target.velocity;
        let mut t;
        for _ in 0..LOOPS {
            t = (r.length() - r_prev.length()) / BULLET_SPEED;
            r_prev = r;
            r += v * t;
        }
        r.y.atan2(r.x)
    }

    // Utility function for drawing time-of-flight lines.
    // Improve (by feeding in &Target) or deprecate.
    //pub fn time(base: Vec2, target: &Target) -> f64 {
    //    let r = target.position - base;
    //    r.length() / BULLET_SPEED
    //}
}

#[derive(Clone, Copy, Debug)]
pub struct Target {
    position: Vec2,
    velocity: Vec2
}

pub struct Ship{
    pid: PID::PID
}

impl Ship {

    pub fn new() -> Self {
        Self {
            pid: PID::PID::new()
        }
    }

    pub fn tick(&mut self) {

        let target = Target{position: target(), velocity: target_velocity()};
        let theta = Solver::solve(position(), target);
        draw_line(position(), Vec2{x: 200.0 * heading().cos(),
                                   y: 200.0 * heading().sin()},
                    0x4444ff);
        let err = angle_diff(theta, heading());
        draw_line(position(), Vec2{x: 200.0 * err * heading().sin(),
                                   y: -200.0 * err * heading().cos(),},
                    0xbbbbff);

        turn(self.pid.pid(err));

        if err.abs() < ANGULAR_TOLERANCE {
            fire(0);
        }
    }
}

