use rand::Rng;
use wasm_bindgen::prelude::*;
use serde::{ Serialize, Deserialize };
use core::ops;
use std::{ f64::consts::PI, vec };
use nalgebra::{ DMatrix, DVector, LU };
// extern crate console_error_panic_hook;
// use std::panic;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, PartialEq, Copy, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn divide(&mut self, n: f64) {
        self.x = self.x / n;
        self.y = self.y / n;
    }
    pub fn distance_from(&self, other: Vec2) -> f64 {
        f64::sqrt(f64::powi(self.x - other.x, 2) + f64::powi(self.y - other.y, 2))
    }
}
impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, d: f64) {
        self.x /= d;
        self.y /= d;
    }
}
impl ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, m: f64) -> Self {
        Self::new(self.x * m, self.y * m)
    }
}
impl ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, m: f64) -> Self {
        Self::new(self.x / m, self.y / m)
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Rod {
    pub length: f64,
    pub mass: f64,
    pub color: u32,
}
#[wasm_bindgen]
impl Rod {
    #[wasm_bindgen(constructor)]
    pub fn new(length: f64, mass: f64, color: u32) -> Rod {
        Rod { length, mass, color }
    }
    pub fn update_length(&mut self, length: f64) {
        self.length = length;
    }
    pub fn get_data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Trail {
    pub pos: Vec2,
    pub color: u32,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Ball {
    pub pos: Vec2,
    pub omega: f64,
    pub theta: f64,
    pub rod: Rod,
    trail: Vec<Trail>,
    pub radius: i32,
    pub mass: f64,
    pub color: u32,
}
#[wasm_bindgen]
impl Ball {
    #[wasm_bindgen(constructor)]
    pub fn new(
        px: f64,
        py: f64,
        omega: f64,
        theta: f64,
        rl: f64,
        rm: f64,
        rc: u32,
        radius: i32,
        mass: f64,
        color: u32
    ) -> Ball {
        Ball {
            pos: Vec2::new(px, py),
            omega,
            theta,
            radius,
            mass,
            color,
            rod: Rod::new(rl, rm, rc),
            trail: vec![],
        }
    }

    pub fn get_trail(&self) -> Vec<Trail> {
        self.trail.clone()
    }

    pub fn add_trail_point(&mut self, pos: Vec2, color: u32, max: usize) {
        self.trail.push(Trail { pos, color });
        if self.trail.len() > max {
            self.trail.remove(0);
        }
    }

    pub fn get_data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Implementation {
    Euler,
    RK4,
    Verlet, // Verlet integration (position-based)
    Leapfrog, // Leapfrog integration (velocity half-steps)
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Universe {
    balls: Vec<Ball>,
    gravity: f64,
    mass_calculation: bool,
    show_trails: bool,
    is_paused: bool,
    implementation: Implementation,
    speed: f64,
    max_balls: usize,
    initial_energy: f64,
    default_mass: f64,
    limit_total_energy: bool,
}
#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Universe {
        // panic::set_hook(Box::new(console_error_panic_hook::hook));
        // wasm_logger::init(wasm_logger::Config::default());
        // log::info!("Universe Init!");
        //  self.x_1 = self.origin_x + self.length_rod_1 * math.sin(self.theta_1)
        // self.y_1 = self.origin_y + self.length_rod_1 * math.cos(self.theta_1)
        // self.x_2 = self.x_1 + self.length_rod_2 * math.sin(self.theta_2)
        // self.y_2 = self.y_1 + self.length_rod_2 * math.cos(self.theta_2)
        let ball1 = Ball::new(100.0, 0.0, 0.0, PI / 2.0, 100.0, 10.0, 0x0f0f0f, 10, 10.0, 0xff0000);
        let ball2 = Ball::new(200.0, 0.0, 0.0, PI / 2.0, 100.0, 10.0, 0x0f0f0f, 10, 10.0, 0x0000ff);
        let mut universe = Universe {
            balls: vec![ball1, ball2],
            gravity: 9.8,
            implementation: Implementation::Euler,
            speed: 1.0 / 20.0,
            mass_calculation: true,
            show_trails: true,
            max_balls: 100,
            is_paused: false,
            initial_energy: 0.0, // Will be calculated next
            default_mass: 10.0, // Default mass used when mass_calculation is false
            limit_total_energy: false, // Enable energy limiting off by default
        };
        // Calculate initial total energy (potential + kinetic)
        universe.initial_energy =
            universe.calculate_potential_energy() + universe.calculate_kinetic_energy();
        universe
    }
    pub fn time_step(&mut self, dt: f64) -> u8 {
        if self.balls.is_empty() || self.is_paused {
            return 1;
        }

        if self.balls.len() > self.max_balls {
            // cutoff for Euler method, remove extras
            self.balls.truncate(self.max_balls);
        }

        // Calculate the effective speed multiplier
        let speed_multiplier = self.speed * 2.0;

        // Instead of scaling dt directly (which causes instability at high speeds),
        // we take multiple smaller steps to maintain numerical stability
        let steps = (speed_multiplier.abs() * 50.0).ceil().max(1.0) as usize;
        let sub_dt = (dt * speed_multiplier) / (steps as f64);

        // Perform multiple substeps with smaller dt
        for _ in 0..steps {
            let result = self.single_physics_step(sub_dt);
            if result != 0 {
                return result; // Early exit if NaN detected
            }
        }

        // Add trail points only once per frame (not per substep)
        if self.show_trails {
            for ball in &mut self.balls {
                ball.add_trail_point(ball.pos, ball.color, 250);
            }
        }
        return 0;
    }

    // Normalize angle to [-PI, PI] range for better floating point precision
    fn normalize_angle(angle: f64) -> f64 {
        let mut a = angle % (2.0 * PI);
        if a > PI {
            a -= 2.0 * PI;
        } else if a < -PI {
            a += 2.0 * PI;
        }
        a
    }

    // Calculate total potential energy of the system
    // U = -sum(m_i * g * y_i) where y_i is the vertical position of each mass
    fn calculate_potential_energy(&self) -> f64 {
        let mut potential = 0.0;
        let mut y_cumulative = 0.0;

        for i in 0..self.balls.len() {
            // Vertical position is cumulative (each mass hangs from the previous)
            y_cumulative += self.balls[i].rod.length * f64::cos(self.balls[i].theta);

            // Potential energy (negative because positive y is down)
            let mass = if self.mass_calculation { self.balls[i].mass } else { self.default_mass };
            potential -= mass * self.gravity * y_cumulative;
        }
        potential
    }

    // Calculate total kinetic energy of the system
    fn calculate_kinetic_energy(&self) -> f64 {
        let mut kinetic = 0.0;

        for i in 0..self.balls.len() {
            // For a multi-pendulum, kinetic energy includes contributions from all previous segments
            let mut vx = 0.0;
            let mut vy = 0.0;

            for j in 0..=i {
                vx +=
                    self.balls[j].rod.length * self.balls[j].omega * f64::cos(self.balls[j].theta);
                vy +=
                    -self.balls[j].rod.length * self.balls[j].omega * f64::sin(self.balls[j].theta);
            }

            let v_squared = vx * vx + vy * vy;

            let mass = if self.mass_calculation { self.balls[i].mass } else { self.default_mass };
            kinetic += 0.5 * mass * v_squared;
        }
        kinetic
    }

    // Constrain velocities based on energy conservation
    fn constrain_velocities(&mut self, initial_energy: f64) {
        let current_potential = self.calculate_potential_energy();
        let max_kinetic = initial_energy - current_potential;

        // If we have negative kinetic energy, we have a problem
        if max_kinetic < 0.0 {
            return;
        }

        let current_kinetic = self.calculate_kinetic_energy();

        // If kinetic energy exceeds what's possible, scale down velocities
        if current_kinetic > max_kinetic && current_kinetic > 0.0 {
            let scale_factor = f64::sqrt(max_kinetic / current_kinetic);
            for i in 0..self.balls.len() {
                self.balls[i].omega *= scale_factor;
            }
        }
    }

    // Recalculate and store the initial energy (call after modifying the system)
    fn update_initial_energy(&mut self) {
        self.initial_energy = self.calculate_potential_energy() + self.calculate_kinetic_energy();
    }

    fn single_physics_step(&mut self, dt: f64) -> u8 {
        if self.implementation == Implementation::Euler {
            let thetas: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.theta)
            );
            let theta_dots: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.omega)
            );

            // Calculate accelerations using the matrix method
            let (_, theta_ddots) = self.calculate_accelerations(&thetas, &theta_dots);

            // Check for NaN before updating
            if theta_ddots.iter().any(|&x| x.is_nan()) {
                return 1;
            }

            // Euler integration: update velocities and positions
            for i in 0..self.balls.len() {
                self.balls[i].omega += theta_ddots[i] * dt;
                self.balls[i].theta += self.balls[i].omega * dt;

                // Normalize angle to [-PI, PI] for better floating point precision
                // self.balls[i].theta = Self::normalize_angle(self.balls[i].theta);

                // Calculate positions (cumulative from origin)
                let mut x = 0.0;
                let mut y = 0.0;
                for j in 0..=i {
                    x += self.balls[j].rod.length * f64::sin(self.balls[j].theta);
                    y += self.balls[j].rod.length * f64::cos(self.balls[j].theta);
                }
                self.balls[i].pos.x = x;
                self.balls[i].pos.y = y;
            }
        } else if self.implementation == Implementation::RK4 {
            let thetas: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.theta)
            );
            let theta_dots: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.omega)
            );

            let k1 = self.calculate_accelerations(&thetas, &theta_dots);
            let k2 = self.calculate_accelerations(
                &(thetas.clone() + &k1.0 * (0.5 * dt)),
                &(theta_dots.clone() + &k1.1 * (0.5 * dt))
            );
            let k3 = self.calculate_accelerations(
                &(thetas.clone() + &k2.0 * (0.5 * dt)),
                &(theta_dots.clone() + &k2.1 * (0.5 * dt))
            );
            let k4 = self.calculate_accelerations(
                &(thetas.clone() + &k3.0 * (1.0 * dt)),
                &(theta_dots.clone() + &k3.1 * (1.0 * dt))
            );

            // Calculate deltas: (k1 + 2*k2 + 2*k3 + k4) * dt/6
            let theta_deltas = (&k1.0 + &k2.0 * 2.0 + &k3.0 * 2.0 + &k4.0) * (dt / 6.0);
            let theta_dot_deltas = (&k1.1 + &k2.1 * 2.0 + &k3.1 * 2.0 + &k4.1) * (dt / 6.0);
            // Update balls
            for i in 0..self.balls.len() {
                self.balls[i].theta += theta_deltas[i];
                self.balls[i].omega += theta_dot_deltas[i];

                // Normalize angle to [-PI, PI] for better floating point precision
                // self.balls[i].theta = Self::normalize_angle(self.balls[i].theta);

                // Calculate positions (cumulative from origin)
                let mut x = 0.0;
                let mut y = 0.0;
                for j in 0..=i {
                    x += self.balls[j].rod.length * f64::sin(self.balls[j].theta);
                    y += self.balls[j].rod.length * f64::cos(self.balls[j].theta);
                }
                self.balls[i].pos.x = x;
                self.balls[i].pos.y = y;
            }
        } else if self.implementation == Implementation::Verlet {
            // Verlet integration (position-based with previous and current positions)
            // Based on: x(t+dt) = x(t) + v(t)*dt + 0.5*a(t)*dt^2
            // Then: v(t+dt) = 0.5*(a(t) + a(t+dt))*dt
            let thetas: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.theta)
            );
            let theta_dots: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.omega)
            );

            // Calculate current accelerations
            let (_, theta_ddots) = self.calculate_accelerations(&thetas, &theta_dots);

            if theta_ddots.iter().any(|&x| x.is_nan()) {
                return 1;
            }

            // Update positions: theta_new = theta + omega*dt + 0.5*alpha*dt^2
            let mut new_thetas = DVector::from_element(self.balls.len(), 0.0);
            for i in 0..self.balls.len() {
                new_thetas[i] = thetas[i] + theta_dots[i] * dt + 0.5 * theta_ddots[i] * dt * dt;
            }

            // Calculate new accelerations at new positions
            let (_, theta_ddots_new) = self.calculate_accelerations(&new_thetas, &theta_dots);

            if theta_ddots_new.iter().any(|&x| x.is_nan()) {
                return 1;
            }

            // Update velocities: omega_new = omega + 0.5*(alpha_old + alpha_new)*dt
            for i in 0..self.balls.len() {
                self.balls[i].omega =
                    theta_dots[i] + 0.5 * (theta_ddots[i] + theta_ddots_new[i]) * dt;
                self.balls[i].theta = new_thetas[i];

                // Normalize angle to [-PI, PI] for better floating point precision
                self.balls[i].theta = Self::normalize_angle(self.balls[i].theta);

                // Calculate positions (cumulative from origin)
                let mut x = 0.0;
                let mut y = 0.0;
                for j in 0..=i {
                    x += self.balls[j].rod.length * f64::sin(self.balls[j].theta);
                    y += self.balls[j].rod.length * f64::cos(self.balls[j].theta);
                }
                self.balls[i].pos.x = x;
                self.balls[i].pos.y = y;
            }
        } else if self.implementation == Implementation::Leapfrog {
            // Leapfrog integration (velocity half-steps)
            // Based on: v(t+dt/2) = v(t) + a(t)*dt/2
            //           x(t+dt) = x(t) + v(t+dt/2)*dt
            //           a(t+dt) = acceleration at new position
            //           v(t+dt) = v(t+dt/2) + a(t+dt)*dt/2
            let thetas: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.theta)
            );
            let theta_dots: DVector<f64> = DVector::from_iterator(
                self.balls.len(),
                self.balls.iter().map(|ball| ball.omega)
            );

            // Step 1: Half-step velocity update
            let (_, theta_ddots) = self.calculate_accelerations(&thetas, &theta_dots);

            // Check for NaN before updating
            if theta_ddots.iter().any(|&x| x.is_nan()) {
                return 1;
            }

            let mut theta_dots_half = theta_dots.clone();
            for i in 0..self.balls.len() {
                theta_dots_half[i] += theta_ddots[i] * (dt / 2.0);
            }

            // Step 2: Full-step position update using half-step velocity
            let mut new_thetas = thetas.clone();
            for i in 0..self.balls.len() {
                new_thetas[i] += theta_dots_half[i] * dt;
            }

            // Step 3: Calculate accelerations at new position
            let (_, theta_ddots_new) = self.calculate_accelerations(&new_thetas, &theta_dots_half);

            if theta_ddots_new.iter().any(|&x| x.is_nan()) {
                return 1;
            }

            // Step 4: Complete velocity update with second half-step
            for i in 0..self.balls.len() {
                self.balls[i].theta = new_thetas[i];
                self.balls[i].omega = theta_dots_half[i] + theta_ddots_new[i] * (dt / 2.0);

                // Normalize angle to [-PI, PI] for better floating point precision
                self.balls[i].theta = Self::normalize_angle(self.balls[i].theta);

                // Calculate positions (cumulative from origin)
                let mut x = 0.0;
                let mut y = 0.0;
                for j in 0..=i {
                    x += self.balls[j].rod.length * f64::sin(self.balls[j].theta);
                    y += self.balls[j].rod.length * f64::cos(self.balls[j].theta);
                }
                self.balls[i].pos.x = x;
                self.balls[i].pos.y = y;
            }
        }

        // Apply energy conservation constraint to prevent unbounded energy growth (if enabled)
        if self.limit_total_energy {
            self.constrain_velocities(self.initial_energy);
        }

        return 0;
    }
    fn calculate_accelerations(
        &self,
        thetas: &DVector<f64>,
        theta_dots: &DVector<f64>
    ) -> (DVector<f64>, DVector<f64>) {
        let n = self.balls.len();

        if self.mass_calculation {
            // Extract masses and lengths
            let masses: Vec<f64> = self.balls
                .iter()
                .map(|ball| ball.mass)
                .collect();
            let lengths: Vec<f64> = self.balls
                .iter()
                .map(|ball| ball.rod.length)
                .collect();

            // Build the mass matrix M
            let mut m: DMatrix<f64> = DMatrix::from_element(n, n, 0.0);
            for i in 0..n {
                for j in 0..n {
                    // Sum of masses from max(i,j) to n-1
                    let mass_sum: f64 = (usize::max(i, j)..n).map(|k| masses[k]).sum();

                    m[(i, j)] =
                        mass_sum * lengths[i] * lengths[j] * f64::cos(thetas[i] - thetas[j]);
                }
            }

            let mut v = DVector::from_element(n, 0.0);
            for i in 0..n {
                let mut sum = 0.0;

                for j in 0..n {
                    let mass_sum: f64 = (usize::max(i, j)..n).map(|k| masses[k]).sum();

                    sum -=
                        mass_sum *
                        lengths[i] *
                        lengths[j] *
                        f64::sin(thetas[i] - thetas[j]) *
                        f64::powi(theta_dots[j], 2);
                }

                // Gravitational term
                let mass_sum_i: f64 = (i..n).map(|k| masses[k]).sum();
                sum -= self.gravity * mass_sum_i * lengths[i] * f64::sin(thetas[i]);

                v[i] = sum;
            }

            let lu = LU::new(m);
            let theta_ddots = lu.solve(&v).unwrap_or_else(|| DVector::from_element(n, 0.0));

            (theta_dots.clone(), theta_ddots)
        } else {
            // Use default mass for all balls when mass_calculation is false
            let masses: Vec<f64> = vec![self.default_mass; n];
            let lengths: Vec<f64> = self.balls
                .iter()
                .map(|ball| ball.rod.length)
                .collect();

            // Build the mass matrix M (same as mass_calculation=true, but with default_mass)
            let mut m: DMatrix<f64> = DMatrix::from_element(n, n, 0.0);
            for i in 0..n {
                for j in 0..n {
                    // Sum of masses from max(i,j) to n-1
                    let mass_sum: f64 = (usize::max(i, j)..n).map(|k| masses[k]).sum();

                    m[(i, j)] =
                        mass_sum * lengths[i] * lengths[j] * f64::cos(thetas[i] - thetas[j]);
                }
            }

            // Build the force vector
            let mut v = DVector::from_element(n, 0.0);
            for i in 0..n {
                let mut sum = 0.0;

                for j in 0..n {
                    let mass_sum: f64 = (usize::max(i, j)..n).map(|k| masses[k]).sum();

                    sum -=
                        mass_sum *
                        lengths[i] *
                        lengths[j] *
                        f64::sin(thetas[i] - thetas[j]) *
                        f64::powi(theta_dots[j], 2);
                }

                // Gravitational term
                let mass_sum_i: f64 = (i..n).map(|k| masses[k]).sum();
                sum -= self.gravity * mass_sum_i * lengths[i] * f64::sin(thetas[i]);

                v[i] = sum;
            }

            // Solve M * theta_ddot = v for theta_ddot
            let lu = LU::new(m);
            let theta_ddots = lu.solve(&v).unwrap_or_else(|| DVector::from_element(n, 0.0));

            (theta_dots.clone(), theta_ddots)
        }
    }
    pub fn reset(&mut self) {
        *self = Universe::new();
    }
    pub fn add_ball(
        &mut self,
        px: f64,
        py: f64,
        omega: f64,
        theta: f64,
        rl: f64,
        rm: f64,
        rc: u32,
        radius: i32,
        mass: f64,
        color: u32
    ) {
        self.balls.push(Ball::new(px, py, omega, theta, rl, rm, rc, radius, mass, color));
        self.update_initial_energy();
    }

    pub fn random_color() -> u32 {
        // Generate a random color in ff0000,0000ff,00ff00,f0f000,00f0f0,f000f0
        let colors = [0xff0000, 0x0000ff, 0x00ff00, 0xf0f000, 0x00f0f0, 0xf000f0];
        colors[rand::rng().random_range(0..colors.len())]
    }
    pub fn add_ball_simple(&mut self, theta: f64) {
        let default_length = 100.0;
        let default_mass = 10.0;
        let default_color = Self::random_color();
        let default_rod_color = 0x0f0f0f;

        // Calculate position from previous ball or origin
        let (px, py) = if let Some(last_ball) = self.balls.last() {
            (
                last_ball.pos.x + default_length * f64::sin(theta),
                last_ball.pos.y + default_length * f64::cos(theta),
            )
        } else {
            (default_length * f64::sin(theta), default_length * f64::cos(theta))
        };

        self.balls.push(
            Ball::new(
                px,
                py,
                0.0,
                theta,
                default_length,
                default_mass,
                default_rod_color,
                10,
                default_mass,
                default_color
            )
        );
        self.update_initial_energy();
    }
    pub fn remove_ball(&mut self) {
        self.balls.pop();
        self.update_initial_energy();
    }
    pub fn get_balls(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.balls).unwrap()
    }

    pub fn get_ball(&self, index: usize) -> Option<Ball> {
        self.balls.get(index).cloned()
    }
    pub fn get_ball_count(&self) -> i32 {
        self.balls.len() as i32
    }

    pub fn update_ball_theta(&mut self, index: usize, theta: f64) {
        if index < self.balls.len() {
            self.balls[index].theta = theta;

            // Recalculate positions for this ball and all subsequent balls
            for i in index..self.balls.len() {
                let (mut x, mut y) = if i == 0 {
                    (0.0, 0.0)
                } else {
                    (self.balls[i - 1].pos.x, self.balls[i - 1].pos.y)
                };

                x += self.balls[i].rod.length * f64::sin(self.balls[i].theta);
                y += self.balls[i].rod.length * f64::cos(self.balls[i].theta);

                self.balls[i].pos.x = x;
                self.balls[i].pos.y = y;
            }
            self.update_initial_energy();
        }
    }

    pub fn update_ball_length(&mut self, index: usize, length: f64) {
        if index < self.balls.len() {
            self.balls[index].rod.length = length;
            // Recalculate positions for this ball and all subsequent balls
            self.update_ball_theta(index, self.balls[index].theta);
            // update_ball_theta already calls update_initial_energy
        }
    }

    pub fn update_ball_mass(&mut self, index: usize, mass: f64) {
        if index < self.balls.len() {
            self.balls[index].mass = mass;
            self.update_initial_energy();
        }
    }

    pub fn update_ball_color(&mut self, index: usize, color: u32) {
        if index < self.balls.len() {
            self.balls[index].color = color;
        }
    }

    pub fn update_ball_radius(&mut self, index: usize, radius: i32) {
        if index < self.balls.len() {
            self.balls[index].radius = radius;
        }
    }

    pub fn update_ball_omega(&mut self, index: usize, omega: f64) {
        if index < self.balls.len() {
            self.balls[index].omega = omega;
            self.update_initial_energy();
        }
    }

    pub fn get_trails(&self) -> JsValue {
        if self.show_trails {
            let trails: Vec<Vec<Trail>> = self.balls
                .iter()
                .map(|ball| ball.trail.clone())
                .collect();
            serde_wasm_bindgen::to_value(&trails).unwrap()
        } else {
            let trails: Vec<Vec<Trail>> = vec![];
            serde_wasm_bindgen::to_value(&trails).unwrap()
        }
    }
    pub fn set_gravity(&mut self, gravity: f64) {
        self.gravity = gravity;
    }
    pub fn get_gravity(&self) -> f64 {
        return self.gravity;
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
    pub fn get_speed(&self) -> f64 {
        return self.speed;
    }

    pub fn set_is_paused(&mut self, is_paused: bool) {
        self.is_paused = is_paused;
    }

    pub fn get_is_paused(&self) -> bool {
        return self.is_paused;
    }

    pub fn set_implementation(&mut self, implementation: Implementation) {
        self.implementation = implementation;
    }
    pub fn get_implementation(&self) -> Implementation {
        return self.implementation;
    }

    pub fn set_mass_calculation(&mut self, mass_calculation: bool) {
        self.mass_calculation = mass_calculation;
    }

    pub fn get_mass_calculation(&self) -> bool {
        return self.mass_calculation;
    }

    pub fn toggle_mass_calculation(&mut self) {
        self.mass_calculation = !self.mass_calculation;
    }

    pub fn set_show_trails(&mut self, show_trails: bool) {
        self.show_trails = show_trails;
    }

    pub fn get_show_trails(&self) -> bool {
        return self.show_trails;
    }

    pub fn toggle_show_trails(&mut self) {
        self.show_trails = !self.show_trails;
    }

    pub fn set_limit_total_energy(&mut self, limit_total_energy: bool) {
        self.limit_total_energy = limit_total_energy;
    }

    pub fn get_limit_total_energy(&self) -> bool {
        return self.limit_total_energy;
    }

    pub fn toggle_limit_total_energy(&mut self) {
        self.limit_total_energy = !self.limit_total_energy;
    }
}
