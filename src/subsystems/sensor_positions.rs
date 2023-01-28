use std::{f32::consts::PI, time::SystemTime};

use crossbeam::channel::{Receiver, Sender};

use crate::components::constants::{AXLE_DIST, B_ISD, MAZE_COL_WIDTH, S_ISD};

#[derive(Debug)]
pub struct SensorPosComputer {
    time: SystemTime,
    prev_angular_velocity: f32,
    x: f32,
    y: f32,
    angle: f32,
    sensor_rads: [(f32, f32); 5],
}

impl SensorPosComputer {
    pub fn new(init_x: f32, init_y: f32) -> Self {
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
            time: SystemTime::now(),
            prev_angular_velocity: 0.,
            x: init_x,
            y: init_y,
            angle: PI / 2.0,
            sensor_rads,
        }
    }

    pub fn compute_pos(
        &mut self,
        wheel_info: Receiver<(i16, i16)>,
        positions_ss: Sender<[(f32, f32); 5]>,
        positions_gui: Sender<[(f32, f32); 5]>,
    ) {
        for (left_speed, right_speed) in wheel_info {
            println!("got wheels");

            // if left_speed == -right_speed {
            //     println!("rotating");
            // }

            let elapsed_time = self.time.elapsed().unwrap().as_millis() as f32 / 1_000.0; // s

            self.time = SystemTime::now();

            let angular_velocity =
                ((right_speed - left_speed) as f32 / 1_000.0) / (AXLE_DIST as f32 / 1_000.0);

            let linear_velocity = ((right_speed + left_speed) as f32 / 1_000.0) / 2.0;

            // trapezoidal rule for integrals
            self.angle += elapsed_time * ((self.prev_angular_velocity + angular_velocity) / 2.0);
            self.x += elapsed_time * linear_velocity * self.angle.cos();
            self.y += elapsed_time * linear_velocity * self.angle.sin();
            //println!("sending ({}, {})", self.x, self.y);

            // update sensor_positions
            let sens_positions: Vec<(f32, f32)> = self
                .sensor_rads
                .iter()
                .map(|(rad, a)| {
                    (
                        (self.x + ((*rad) * (self.angle + a).cos()) as f32)
                            * (MAZE_COL_WIDTH / 0.2),
                        (self.y + ((*rad) * (self.angle + a).sin()) as f32)
                            * (MAZE_COL_WIDTH / 0.2),
                    )
                })
                .collect();

            self.prev_angular_velocity = angular_velocity;

            let sensor_final_positions: Result<[(f32, f32); 5], Vec<(f32, f32)>> =
                sens_positions.try_into();

            if let Ok(sens_positions) = sensor_final_positions {
                if positions_ss.send(sens_positions).is_err() {
                    panic!("FATAL: position could not be sent to SS!");
                }

                if positions_gui.send(sens_positions).is_err() {
                    panic!("FATAL: position could not be sent to GUI!");
                }
            }
        }

        println!("compute pos end");
    }
}
