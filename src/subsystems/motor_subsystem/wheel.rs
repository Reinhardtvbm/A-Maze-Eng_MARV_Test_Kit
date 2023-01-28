use std::{f32::consts::PI, time::SystemTime};

use crate::components::constants;

#[derive(Debug)]
pub struct Wheels {
    left_speed: i16,
    right_speed: i16,
    left_distance: f32,
    right_distance: f32,
    rotation: f32,
    total_distance: f32,
    _axle_dist: f32,
    time: SystemTime,
}

impl Wheels {
    pub fn new(axle_distance: f32) -> Self {
        Self {
            left_speed: 0,
            right_speed: 0,
            left_distance: 0.0,
            right_distance: 0.0,
            rotation: 0.0,
            total_distance: 0.0,
            _axle_dist: axle_distance,
            time: SystemTime::now(),
        }
    }

    pub fn set_left_wheel_speed(&mut self, speed: i16) {
        self.left_speed = speed;
    }

    pub fn set_right_wheel_speed(&mut self, speed: i16) {
        self.right_speed = speed;
    }

    pub fn get_left_wheel_speed(&self) -> u8 {
        self.left_speed.unsigned_abs() as u8
    }

    pub fn get_right_wheel_speed(&self) -> u8 {
        self.right_speed.unsigned_abs() as u8
    }

    pub fn going_forward(&self) -> bool {
        ((self.left_speed + self.right_speed) / 2) > 0
    }

    pub fn left_rotation(&self) -> bool {
        self.right_speed > 0
    }

    pub fn get_distance(&self) -> u16 {
        self.total_distance.abs() as u16
    }

    pub fn get_rotation(&self) -> u16 {
        (self.rotation.abs() * (180.0 / PI)).floor() as u16
    }

    pub fn get_left(&self) -> i16 {
        self.left_speed
    }

    pub fn get_right(&self) -> i16 {
        self.right_speed
    }

    pub fn update_distance(&mut self) {
        if self.left_speed == 0 && self.right_speed == 0 {
            self.reset_fields();
        } else {
            // get the elapsed time and reset it
            let time = self.time.elapsed().unwrap().as_secs_f32();
            self.time = SystemTime::now();

            // store speeds as f32 so that the conversion from i16 only has to be done once
            let left_speed = self.left_speed as f32;
            let right_speed = self.right_speed as f32;

            // calculate the linear and angular velocity of the robot
            let linear_speed = (left_speed + right_speed) / 2.0;
            let angular_velocity = (right_speed - left_speed) / (constants::AXLE_DIST as f32);

            // update the distances that each wheel has travelled respectively (first order numerical integration / rectangle rule)
            self.left_distance += time * left_speed;
            self.right_distance += time * right_speed;

            // update the total linear distance travelled by the robot and its angle/rotation (first order numerical integration / rectangle rule)
            self.total_distance += time * linear_speed;
            self.rotation += time * angular_velocity;
        }
    }

    fn reset_fields(&mut self) {
        self.left_distance = 0.0;
        self.rotation = 0.0;
        self.right_distance = 0.0;
        self.total_distance = 0.0;
    }
}
