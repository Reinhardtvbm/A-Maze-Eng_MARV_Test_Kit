use std::{f32::consts::PI, time::SystemTime};

use crate::{
    asynchronous::{one_to_many_channel::OTMChannel, one_to_one_channel::OTOChannel},
    components::constants::{AXLE_DIST, B_ISD, MAZE_COL_WIDTH, S_ISD},
};

#[derive(Debug, Clone, Copy)]
pub struct Speeds(f32, f32);

impl Speeds {
    pub fn new(left_speed: f32, right_speed: f32) -> Self {
        Self(left_speed, right_speed)
    }

    pub fn left_speed(&self) -> f32 {
        self.0
    }

    pub fn right_speed(&self) -> f32 {
        self.1
    }
}

pub struct RobotParams {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

pub struct CalcParams {
    pub time: SystemTime,
    pub prev_angular_velocity: f32,
    pub sensor_rads: [(f32, f32); 5],
}

pub struct SensorPosComputer {
    robot_parameters: RobotParams,
    calculation_parameters: CalcParams,
    in_channel: OTOChannel<Speeds>,
    out_channel: OTMChannel<[(f32, f32); 5]>,
}

impl SensorPosComputer {
    pub fn new(
        init_x: f32,
        init_y: f32,
        in_channel: OTOChannel<Speeds>,
        out_channel: OTMChannel<[(f32, f32); 5]>,
    ) -> Self {
        let inside_rad: f32 =
            ((AXLE_DIST as f32).powi(2) + (S_ISD as f32).powi(2)).sqrt() / 1_000.0;
        let outside_rad: f32 =
            ((AXLE_DIST as f32).powi(2) + (S_ISD as f32 + B_ISD as f32).powi(2)).sqrt() / 1_000.0;

        let inside_angle: f32 = ((S_ISD as f32 / 1_000.0) / inside_rad).asin();
        let outside_angle: f32 = (((S_ISD as f32 + B_ISD as f32) / 1_000.0) / outside_rad).asin();

        let sensor_rads: [(f32, f32); 5] = [
            (outside_rad, -outside_angle),
            (inside_rad, -inside_angle),
            (AXLE_DIST as f32 / 1_000.0, 0.0),
            (inside_rad, inside_angle),
            (outside_rad, outside_angle),
        ];

        Self {
            calculation_parameters: CalcParams {
                time: SystemTime::now(),
                prev_angular_velocity: 0.,
                sensor_rads,
            },
            robot_parameters: RobotParams {
                x: init_x,
                y: init_y,
                angle: PI / 2.0,
            },
            in_channel,
            out_channel,
        }
    }

    pub fn compute_pos(&mut self) {
        loop {
            // receive wheel speeds and compute the sensor positions
            let speeds = self.in_channel.receive();
            let sensor_positions = self.compute(speeds);

            // send them to the GUI
            self.out_channel.send(sensor_positions);
        }
    }

    fn compute(&mut self, speeds: Speeds) -> [(f32, f32); 5] {
        let elapsed_time = self
            .calculation_parameters
            .time
            .elapsed()
            .unwrap()
            .as_millis() as f32
            / 1_000.0; // s

        let right_speed = speeds.right_speed();
        let left_speed = speeds.left_speed();

        self.calculation_parameters.time = SystemTime::now();

        let angular_velocity =
            ((right_speed - left_speed) / 1_000.0) / (AXLE_DIST as f32 / 1_000.0);

        let linear_velocity = ((right_speed + left_speed) / 1_000.0) / 2.0;

        // trapezoidal rule for integrals
        self.robot_parameters.angle += elapsed_time
            * ((self.calculation_parameters.prev_angular_velocity + angular_velocity) / 2.0);
        self.robot_parameters.x +=
            elapsed_time * linear_velocity * self.robot_parameters.angle.cos();
        self.robot_parameters.y +=
            elapsed_time * linear_velocity * self.robot_parameters.angle.sin();

        // update sensor_positions
        let mut sensor_positions = [(0.0, 0.0); 5];

        self.calculation_parameters
            .sensor_rads
            .iter()
            .enumerate()
            .for_each(|(index, (radius, angle))| {
                sensor_positions[index] = (
                    (self.robot_parameters.x
                        + ((*radius) * (self.robot_parameters.angle + angle).cos()) as f32)
                        * (MAZE_COL_WIDTH / 0.2),
                    (self.robot_parameters.y
                        + ((*radius) * (self.robot_parameters.angle + angle).sin()) as f32)
                        * (MAZE_COL_WIDTH / 0.2),
                )
            });

        self.calculation_parameters.prev_angular_velocity = angular_velocity;

        sensor_positions
    }
}
