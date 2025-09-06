use ::glam::Vec2;
use macroquad::prelude::*;
use rayon::prelude::*;

// window size
const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

// scientific constants
const G: f64 = 6.6743e-11;
const SPEED_OF_LIGHT: f64 = 299792458.0;
const BH_MASS: f64 = 3e28;

// window configuration
fn window_config() -> Conf {
    Conf {
        window_title: String::from("Gargantua Dev Build"),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

//struct for Black Hole
struct BlackHole {
    position: Vec2,
    mass: f64,
    r_s: f64,
}

impl BlackHole {
    fn calc_r_s(&mut self) {
        self.r_s = (2.0 * G * self.mass) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
    }

    fn draw(&self) {
        let center_x = self.position.x;
        let center_y = self.position.y;
        let radius = self.r_s as f32;
        
        // accretion disk
        for i in 0..10 {
            let current_radius = radius * (1.0 + i as f32 * 0.08);
            let alpha = 0.9 - (i as f32 * 0.05);
            
            // Gradient from deep red/orange to bright orange - less tight color transitions
            let red_component = 1.0;
            let green_component = 0.1 + (i as f32 * 0.08); // Faster transition to orange
            let blue_component = i as f32 * 0.02; // More noticeable blue shift in outer layers
            
            draw_circle(
                center_x, 
                center_y, 
                current_radius, 
                Color::new(red_component, green_component, blue_component, alpha)
            );
        }
        
        // Event horizon - pure black
        draw_circle(center_x, center_y, radius, BLACK);
        
        // Optional: Thin bright ring at event horizon for definition
        draw_circle_lines(center_x, center_y, radius, 2.0, ORANGE);
    }
}

// struct for Ray
#[derive(Default)]
struct Ray {
    x: f64,
    y: f64,
    r: f64,
    phi: f64,
    dr: f64,
    dphi: f64,
    trail: Vec<Vec2>,
    e: f64, // Energy constant
    disabled: bool,
}

// implementation of the geodesic right-hand side function
fn geodesic_rhs(ray: &Ray, rs: f64) -> [f64; 4] {
    let r = ray.r;
    let dr = ray.dr;
    let dphi = ray.dphi;
    let e = ray.e;

    let f = 1.0 - rs / r;

    // dr/dλ = dr
    let rhs_0 = dr;

    // dφ/dλ = dphi
    let rhs_1 = dphi;

    // d²r/dλ² from Schwarzschild null geodesic:
    let dt_dlambda = e / f;
    let rhs_2 = -(rs / (2.0 * r * r)) * f * (dt_dlambda * dt_dlambda)
        + (rs / (2.0 * r * r * f)) * (dr * dr)
        + (r - rs) * (dphi * dphi);

    // d²φ/dλ² = -2*(dr * dphi) / r
    let rhs_3 = -2.0 * dr * dphi / r;

    [rhs_0, rhs_1, rhs_2, rhs_3]
}

impl Ray {
    fn get_polar(&mut self, bh: &BlackHole) {
        self.r = ((self.x - bh.position.x as f64).powi(2)
            + (self.y - (bh.position.y as f64)).powi(2))
        .sqrt();
        self.phi = (self.y - bh.position.y as f64).atan2(self.x - bh.position.x as f64);
    }

    fn draw(&self) {
        for i in 1..self.trail.len() {
            draw_rectangle(
                self.trail[i].x,
                self.trail[i].y,
                3.,
                3.,
                Color {
                    r: 120.,
                    g: 120.,
                    b: 120.,
                    a: 0.4,
                },
            );
        }
        if !self.disabled {
            draw_rectangle(
                self.x as f32,
                self.y as f32,
                5.,
                5.,
                Color {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 0.7,
                },
            );
        } else {
            return;
        }
    }

    fn push_back(&mut self) {
        self.trail.push(Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        });
    }

    fn geodesic(&mut self, bh: &BlackHole) {
        // Use the Rust implementation of geodesic_rhs
        let rhs = geodesic_rhs(self, bh.r_s);

        // Simple Euler integration step
        let dt = 0.1;

        // Update velocities
        self.dr += rhs[2] * dt;
        self.dphi += rhs[3] * dt;

        // Update positions
        self.r += self.dr * dt;
        self.phi += self.dphi * dt;
    }

    fn step(&mut self, bh: &BlackHole, _lambda: f64, screen_width: f64, screen_height: f64) {
        if self.disabled == true {
            return;
        }
        self.get_polar(bh);

        // if the ray is within the schwarzschild radius: STOP
        if self.r < bh.r_s || self.x > screen_width || self.y > screen_height {
            self.disabled = true;
            self.trail.pop();
            return;
        }

        self.geodesic(bh);

        // Convert back to Cartesian coordinates relative to black hole
        self.x = bh.position.x as f64 + self.r * self.phi.cos();
        self.y = bh.position.y as f64 + self.r * self.phi.sin();

        self.push_back();
    }

    fn initialize_velocity(&mut self, bh: &BlackHole, initial_velocity: Vec2) {
        self.get_polar(bh);

        // Convert Cartesian velocity to polar coordinates
        let dx = initial_velocity.x as f64;
        let dy = initial_velocity.y as f64;

        let cos_phi = self.phi.cos();
        let sin_phi = self.phi.sin();

        // Transform velocity to polar coordinates
        self.dr = dx * cos_phi + dy * sin_phi;
        self.dphi = (-dx * sin_phi + dy * cos_phi) / self.r;

        // Set energy constant (conserved quantity)
        let f = 1.0 - bh.r_s / self.r;
        self.e = f;
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut gargantua = BlackHole {
        position: Vec2 {
            x: (WIDTH / 2) as f32,
            y: (HEIGHT / 2) as f32,
        },
        mass: 2. * BH_MASS,
        r_s: 0.,
    };

    // Parallel ray creation from single point at top left corner
    let num_rays = 50; // Number of rays to create
    let start_x = 50.0; // Top left corner x position
    let start_y = 50.0; // Top left corner y position
    
    let rays: Vec<Ray> = (0..num_rays)
        .into_par_iter()
        .map(|i| {
            let mut ray = Ray {
                x: start_x,
                y: start_y,
                e: 1.0, // Initialize energy constant
                ..Default::default()
            };

            // Create rays with slightly different angles for spread pattern
            let angle_offset = (i as f64 - num_rays as f64 / 2.0) * 0.05; // Small angle variations
            let velocity_x = 10.0 * angle_offset.cos() + 10.;
            let velocity_y = 10.0 * angle_offset.sin() + 10.;

            // Initialize the ray's velocity properly
            ray.initialize_velocity(&gargantua, Vec2::new(velocity_x as f32, velocity_y as f32));
            ray
        })
        .collect();

    let mut rays = rays; // Make it mutable for the main loop

    // get the schwarzschild radius of the black hole
    gargantua.calc_r_s();

    println!("Schwarzschild radius: {}", gargantua.r_s);

    // main app loop
    loop {
        clear_background(Color::from_rgba(5, 5, 20, 255));

        gargantua.draw();

        let screen_w = screen_width() as f64;
        let screen_h = screen_height() as f64;
        
        // Parallel processing for ray stepping
        rays.par_iter_mut().for_each(|ray| {
            ray.step(&gargantua, 1., screen_w, screen_h);
        });

        // Sequential drawing (required due to macroquad's rendering constraints)
        for ray in &rays {
            ray.draw();
        }
        next_frame().await
    }
}
