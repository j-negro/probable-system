pub const PARTICLE_COUNT: usize = 200;
pub const SIMULATION_LENGHT: f64 = 20.0;

pub const TARGET_SIZE: f64 = 1.2;
pub const TARGET_LEFT_X: f64 = (SIMULATION_LENGHT - TARGET_SIZE) / 2.0; // (20 - 1.2) / 2 = 9.4
pub const TARGET_RIGHT_X: f64 = TARGET_LEFT_X + TARGET_SIZE; // 9.4 + 1.2 = 10.6

pub const MIN_PARTICLE_RADIUS: f64 = 0.1;
pub const MAX_PARTICLE_RADIUS: f64 = 0.37;

pub const MAX_DESIRED_VELOCITY: f64 = 2.0;
pub const BETA: f64 = 0.9;

pub const TIME_STEP: f64 = MIN_PARTICLE_RADIUS / (2.0 * MAX_DESIRED_VELOCITY);
pub const RADIUS_INCREMENT: f64 = MAX_PARTICLE_RADIUS / (0.5 / TIME_STEP);

pub fn calculate_desired_velocity(radius: f64) -> f64 {
    MAX_DESIRED_VELOCITY
        * ((radius - MIN_PARTICLE_RADIUS) / (MAX_PARTICLE_RADIUS - MIN_PARTICLE_RADIUS)).powf(BETA)
}