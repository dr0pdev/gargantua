use glam::Vec2;

// Scientific constants
pub const G: f64 = 6.6743e-11;
pub const SPEED_OF_LIGHT: f64 = 299792458.0;
pub const BH_MASS: f64 = 3e28;

// Calculate Schwarzschild radius
pub fn calculate_schwarzschild_radius() -> f64 {
    (2.0 * G * BH_MASS) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT)
}

// Black Hole structure
#[derive(Debug, Clone)]
pub struct BlackHole {
    pub position: Vec2,
    pub mass: f64,
    pub r_s: f64,
}

impl BlackHole {
    pub fn new(x: f32, y: f32, mass: f64) -> Self {
        let mut bh = BlackHole {
            position: Vec2::new(x, y),
            mass,
            r_s: 0.0,
        };
        bh.calc_r_s();
        bh
    }

    pub fn calc_r_s(&mut self) {
        self.r_s = (2.0 * G * self.mass) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
    }
}

// Ray structure
#[derive(Debug, Clone, Default)]
pub struct Ray {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub phi: f64,
    pub dr: f64,
    pub dphi: f64,
    pub trail: Vec<Vec2>,
    pub e: f64,
    pub disabled: bool,
}

impl Ray {
    pub fn new(x: f64, y: f64) -> Self {
        Ray {
            x,
            y,
            e: 1.0,
            ..Default::default()
        }
    }

    pub fn get_polar(&mut self, bh: &BlackHole) {
        self.r = ((self.x - bh.position.x as f64).powi(2)
            + (self.y - (bh.position.y as f64)).powi(2))
        .sqrt();
        self.phi = (self.y - bh.position.y as f64).atan2(self.x - bh.position.x as f64);
    }

    pub fn push_back(&mut self) {
        self.trail.push(Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        });
    }

    pub fn geodesic(&mut self, bh: &BlackHole) {
        let rhs = geodesic_rhs(self, bh.r_s);
        let dt = 0.1;

        self.dr += rhs[2] * dt;
        self.dphi += rhs[3] * dt;

        self.r += self.dr * dt;
        self.phi += self.dphi * dt;
    }

    pub fn step(&mut self, bh: &BlackHole, screen_width: f64, screen_height: f64) {
        if self.disabled {
            return;
        }
        self.get_polar(bh);

        if self.r < bh.r_s || self.x > screen_width || self.y > screen_height {
            self.disabled = true;
            self.trail.pop();
            return;
        }

        self.geodesic(bh);

        self.x = bh.position.x as f64 + self.r * self.phi.cos();
        self.y = bh.position.y as f64 + self.r * self.phi.sin();

        self.push_back();
    }

    pub fn initialize_velocity(&mut self, bh: &BlackHole, initial_velocity: Vec2) {
        self.get_polar(bh);

        let dx = initial_velocity.x as f64;
        let dy = initial_velocity.y as f64;

        let cos_phi = self.phi.cos();
        let sin_phi = self.phi.sin();

        self.dr = dx * cos_phi + dy * sin_phi;
        self.dphi = (-dx * sin_phi + dy * cos_phi) / self.r;

        let f = 1.0 - bh.r_s / self.r;
        self.e = f;
    }
}

// Geodesic right-hand side function
fn geodesic_rhs(ray: &Ray, rs: f64) -> [f64; 4] {
    let r = ray.r;
    let dr = ray.dr;
    let dphi = ray.dphi;
    let e = ray.e;

    let f = 1.0 - rs / r;

    let rhs_0 = dr;
    let rhs_1 = dphi;

    let dt_dlambda = e / f;
    let rhs_2 = -(rs / (2.0 * r * r)) * f * (dt_dlambda * dt_dlambda)
        + (rs / (2.0 * r * r * f)) * (dr * dr)
        + (r - rs) * (dphi * dphi);

    let rhs_3 = -2.0 * dr * dphi / r;

    [rhs_0, rhs_1, rhs_2, rhs_3]
}

// Simulation state
pub struct Simulation {
    pub black_hole: BlackHole,
    pub rays: Vec<Ray>,
    pub width: i32,
    pub height: i32,
}

impl Simulation {
    pub fn new(width: i32, height: i32) -> Self {
        let black_hole = BlackHole::new(
            (width / 2) as f32,
            (height / 2) as f32,
            2.0 * BH_MASS,
        );

        let num_rays = 50;
        let start_x = 50.0;
        let start_y = 50.0;

        let rays: Vec<Ray> = (0..num_rays)
            .map(|i| {
                // Create a very wide spread pattern for high-resolution canvas
                let spread_angle = (i as f64 / (num_rays - 1) as f64) * 2.0; // 2.0 radians spread (~115°)
                let offset_distance = 120.0; // Very large distance for 800x800 canvas
                
                let ray_start_x = start_x + spread_angle.cos() * offset_distance;
                let ray_start_y = start_y + spread_angle.sin() * offset_distance;

                let mut ray = Ray::new(ray_start_x, ray_start_y);

                let dx = black_hole.position.x as f64 - ray_start_x;
                let dy = black_hole.position.y as f64 - ray_start_y;
                let distance = (dx * dx + dy * dy).sqrt();

                let speed = 10.0;
                let velocity_x = (dx / distance) * speed;
                let velocity_y = (dy / distance) * speed;

                ray.initialize_velocity(&black_hole, Vec2::new(velocity_x as f32, velocity_y as f32));
                ray
            })
            .collect();

        Simulation {
            black_hole,
            rays,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let screen_w = self.width as f64;
        let screen_h = self.height as f64;

        for ray in &mut self.rays {
            ray.step(&self.black_hole, screen_w, screen_h);
        }
    }

    pub fn get_ray_data(&self) -> Vec<(f64, f64, bool)> {
        self.rays.iter()
            .map(|ray| (ray.x, ray.y, ray.disabled))
            .collect()
    }

    pub fn get_trail_data(&self) -> Vec<Vec<(f32, f32)>> {
        self.rays.iter()
            .map(|ray| ray.trail.iter().map(|v| (v.x, v.y)).collect())
            .collect()
    }
}
