// Tutorial: Radar
// Destroy the enemy ships. Use your radar to find them.
// Hint: Press 'g' in-game to show where your radar is looking.
// Hint: Press 'n' to single-step.
// Hint: Use the set_radar_heading() function to keep your radar pointed at a
// target, or to search for a new one.
//
// Some updates to more statefulness in the Ship's main loop to track targets,
// plus DynamicTarget is now derived from ScanResult (very similar to Target)

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
    use super::BULLET_SPEED;
    const LOOPS: u8 = 10;

    pub fn solve(base: Vec2, target: &DynamicTarget) -> f64 {
        let mut r = target.position - base;
        let mut r_prev = Vec2{x:0., y:0.};
        let mut v = target.velocity;
        let mut a;
        let mut t;
        for _ in 0..LOOPS {
            t = (r.length() - r_prev.length()) / BULLET_SPEED;
            r_prev = r;
            v = target.velocity;
            a = target.acceleration;
            r += v * t + 0.5 * a * t.powi(2);
        }
        r.y.atan2(r.x)
    }
}

pub struct DynamicTarget {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2
}

impl DynamicTarget {
    pub fn new(target: ScanResult) -> Self {
        Self {
            position: target.position,
            velocity: target.velocity,
            acceleration: Vec2{x: 0.0, y: 0.0}
        }
    }

    pub fn update(&mut self, new: ScanResult) {
        let old_velocity = self.velocity;
        self.position = new.position;
        self.velocity = new.velocity;
        self.acceleration = self.velocity - old_velocity;
    }
}

pub struct Ship {
    pid: PID::PID,
    target: Option<DynamicTarget>
}

impl Ship {
    pub fn new() -> Self {
        Self {
            pid: PID::PID::new(),
            target: None
        }
    }

    pub fn track(&mut self) {
        set_radar_heading(self.target.position.angle());
        let theta = Solver::solve(position(), &self.target);
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

    pub fn tick(&mut self) {
        if let Some(contact) = scan() {
            if self.target.is_some() {
                self.target.update(contact);
            } else {
                self.target = Some(DynamicTarget::new(contact));
            }
            self.track();
        } else {
            set_radar_heading(radar_heading() + radar_width());
        }
    }
}
