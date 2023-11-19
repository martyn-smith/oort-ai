// Tutorial: Radio
// Destroy the enemy ship. Your radar is broken, but a radio signal on channel
// 2 will give you its position and velocity.
//
// Chance to flex with Traits, although it means currently we're going back to a static Target.
// TODO: fix that.

use oort_api::prelude::*;

use std::convert::From;

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
    use super::BULLET_SPEED;
    use super::Target;
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
            //draw_line(position(), r, 0x00ffff);
        }
        r.y.atan2(r.x)
    }
}

pub struct Target {
    pub position: Vec2,
    pub velocity: Vec2
}

impl From<[f64; 4]> for Target {

    fn from(f: [f64; 4]) -> Self {
        Self {
            position: Vec2{x: f[0], y: f[1]},
            velocity: Vec2{x: f[2], y: f[2]}
        }
    }
}

pub struct Ship{
    pid: PID::PID::new()
}

impl Ship {

    pub fn new() -> Self {
        Self {
            pid: PID::PID::new()
        }
    }

    pub fn tick(&mut self) {
        set_radio_channel(2);
        if let Some(msg) = receive() {
            let target = Target::from(msg);
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

        } else {
            debug!("no message received");
        }
    }
}
