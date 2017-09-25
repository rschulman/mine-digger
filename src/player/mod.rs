use std::f32;
use cgmath::{Point3, Vector3, Zero};
use cgmath::prelude::*;
use time::PreciseTime;

pub struct Player {
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    pub view_dir: f32,
    pub view_azimuth: f32,
    velocity: Vector3<f32>,
    prior_tick: PreciseTime,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x: -6.0,
            pos_y: 0.0,
            pos_z: 0.0,
            view_dir: 3.14,
            view_azimuth: 0.0,
            velocity: Vector3::zero(),
            prior_tick: PreciseTime::now(),
        }
    }

    pub fn accelerate(&mut self, push: Vector3<f32>) {
        self.velocity += push;
        if self.velocity.distance(Vector3::zero()) > 5.0 {
            let angle = (self.velocity.y / self.velocity.x).atan();
            self.velocity = Vector3::new(angle.cos() * 5.0, angle.sin() * 5.0, 0.0);
        }
    }

    pub fn tick(&mut self) {
        let now = PreciseTime::now();
        let delta_dur = self.prior_tick.to(now);
        let delta = delta_dur.num_milliseconds();
        self.pos_x += self.velocity.x * delta as f32;
        self.pos_y += self.velocity.y * delta as f32;
        self.pos_z += self.velocity.z * delta as f32;
        self.prior_tick = now;
    }

    pub fn position(&self) -> Point3<f32> {
        Point3::new(self.pos_x, self.pos_y, self.pos_z)
    }

    pub fn view_obj(&self) -> Vector3<f32> {
        Vector3::new(self.view_azimuth.cos() * self.view_dir.sin(),
                     self.view_azimuth.sin(),
                     self.view_azimuth.cos() * self.view_dir.cos())
    }
}
