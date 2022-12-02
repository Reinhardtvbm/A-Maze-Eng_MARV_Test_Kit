use std::time::SystemTime;

#[derive(Debug)]
pub struct Wheels {
    left_speed: i16,
    right_speed: i16,
    left_distance: f32,
    right_distance: f32,
    rotation: f32,
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
            rotation: 0.0,
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
        self.total_distance as u16
    }

    pub fn get_rotation(&self) -> u16 {
        self.rotation as u16
    }

    pub fn update_distance(&mut self) {
        if self.left_speed == 0 && self.right_speed == 0 {
            self.left_distance = 0.0;
            self.rotation = 0.0;
            self.right_distance = 0.0;
            self.total_distance = 0.0;
        } else {
            let time = self.time.elapsed().unwrap();
            self.time = SystemTime::now();

            self.left_distance += time.as_secs_f32() * (self.left_speed as f32);
            self.right_distance += time.as_secs_f32() * (self.right_speed as f32);

            let linear_speed = (self.left_speed + self.right_speed) as f32 / 2.0;

            self.total_distance += time.as_secs_f32() * linear_speed;

            self.rotation = self.right_distance / (self.axle_dist / 2.0);
        }
    }
}
