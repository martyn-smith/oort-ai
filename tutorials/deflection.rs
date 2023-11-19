// Tutorial: Deflection
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function.
//
// Hint: p = p₀ + v₀t + ½at² (the third equation of kinematics)
//
// Similar to lead in approach, with one key difference:
// we now use a DynamicTarget which is stateful, and stored by the ship,
// (so the ship is *also* stateful). Additionally it means target is passed to the solver by-reference.

use oort_api::prelude::*;
const BULLET_SPEED: f64 = 1000.0; // m/s
const ANGULAR_TOLERANCE: f64 = 0.005; //radians, tunable

mod PID {
    use oort_api::prelude::*;
    const P: f64 = 2.5; //a.u, P component of PID, tunable
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
    use super::{BULLET_SPEED, DynamicTarget};
    const LOOPS: u8 = 10;

    pub fn solve(base: Vec2, target: &DynamicTarget) -> f64 {
        let mut r = target.position - base;
        let mut r_prev = Vec2{x:0., y:0.};
        let mut v = target.velocity;
        let mut v_prev = target.velocity;
        let mut a;
        let mut t;
        for _ in 0..LOOPS {
            t = (r.length() - r_prev.length()) / BULLET_SPEED;
            r_prev = r;
            v_prev = v;
            v = target.velocity;
            a = target.acceleration;
            r += v * t + 0.5 * a * t.powi(2);
        }
        r.y.atan2(r.x)
    }
}

pub struct Target {
    position: Vec2,
    velocity: Vec2
}

pub struct DynamicTarget {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2
}

impl DynamicTarget {
    pub fn new(target: Target) -> Self {
        Self {
            position: target.position,
            velocity: target.velocity,
            acceleration: Vec2{x: 0.0, y: 0.0}
        }
    }

    pub fn update(&mut self, new: Target) {
        let old_velocity = self.velocity;
        self.position = new.position;
        self.velocity = new.velocity;
        self.acceleration = self.velocity - old_velocity;
    }
}

pub struct Ship{
    pid: PID::PID,
    target: DynamicTarget
}

impl Ship {

    pub fn new() -> Self {
        let target = Target{position: target(), velocity: target_velocity()};
        Self {
            pid: PID::PID::new(),
            target: DynamicTarget::new(target)
        }
    }

    pub fn tick(&mut self) {
        self.target.update(Target{position: target(), velocity: target_velocity()});
        let theta = Solver::solve(position(), &self.target);
        //let t = Solver::time(position());
        //draw_line(position(), target(), 0x00ff00);
        //draw_line(position(), Vec2{x: 500.0 * t * theta.cos(),
        //                           y: 500.0 * t * theta.sin()},
        //            0xff4444);
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
            //turn(dh * P);
    }
}
