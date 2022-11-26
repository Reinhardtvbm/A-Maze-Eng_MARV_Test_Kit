use std::time::{Duration, SystemTime};

struct Wheels {
    left_speed: i16,
    right_speed: i16,
    left_distance: f32,
    right_distance: f32,
    total_distance: f32,
    axle_dist: f32,
    time: SystemTime,
}

impl Wheels {
    pub fn new(axle_distance: f32) -> Self {
        Self {
            left_speed: 0,
            right_speed: 0,
            left_distance: 0.0,
            right_distance: 0.0,
            total_distance: 0.0,
            axle_dist: axle_distance,
            time: SystemTime::now(),
        }
    } 
    
    pub fn set_left_wheel_speed(&mut self, speed: i16) {
        self.left_speed = speed;
    }

    pub fn set_right_wheel_speed(&mut self, speed: i16) {
        self.right_speed = speed;
    }

    pub fn update_distance(&mut self) {
        let time = self.time.elapsed().unwrap();
        self.time = SystemTime::now();

        if self.left_speed == 0 && self.right_speed == 0 {
            self.left_distance = 0.0;
            self.right_distance = 0.0;
            self.total_distance = 0.0;
            return;
        }

        self.left_distance += time.as_secs_f32() * (self.left_speed as f32);
        self.right_distance += time.as_secs_f32() * (self.right_speed as f32);

        let linear_speed = (self.left_speed + self.right_speed) as f32 / 2.0;

        self.total_distance += time.as_secs_f32() * linear_speed;
    }
}