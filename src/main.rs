use ::glam::Vec2;
use macroquad::prelude::*;

// window size
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// scientific constants
const G: f64 = 6.6743e-11;
const SPEED_OF_LIGHT: f64 = 299792458.0;

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
        draw_circle(
            self.position.x as f32,
            self.position.y,
            self.r_s as f32,
            RED,
        );
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
    e: f64, // Energy constant (replaces E from C function)
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
        draw_rectangle(self.x as f32, self.y as f32, 5., 5., WHITE);

        for i in 1..self.trail.len() {
            let alpha = (i as f32 / self.trail.len() as f32) * 0.5;
            draw_rectangle(
                self.trail[i].x,
                self.trail[i].y,
                3.,
                3.,
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: alpha,
                },
            );
        }
    }

    fn push_back(&mut self) {
        if self.trail.len() % 3 == 0 {
            self.trail.push(Vec2 {
                x: self.x as f32,
                y: self.y as f32,
            });
        }

        // Keep trail length manageable
        if self.trail.len() > 200 {
            self.trail.remove(0);
        }
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

    fn step(&mut self, bh: &BlackHole, _lambda: f64) {
        self.get_polar(bh);

        // if the ray is within the schwarzschild radius: STOP
        if self.r < bh.r_s {
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
        mass: 6e28,
        r_s: 0.,
    };

    let mut rays: Vec<Ray> = vec![];

    // Create rays starting from the left side
    for i in 0..20 {
        let start_y = (HEIGHT as f64 / 4.0) + (i as f64 * HEIGHT as f64 / 40.0);
        let mut ray = Ray {
            x: 50.0,
            y: start_y,
            e: 1.0, // Initialize energy constant
            ..Default::default()
        };

        // Initialize the ray's velocity properly
        ray.initialize_velocity(&gargantua, Vec2::new(10.0, 0.0));
        rays.push(ray);
    }

    // get the schwarzschild radius of the black hole
    gargantua.calc_r_s();

    println!("Schwarzschild radius: {}", gargantua.r_s);

    // main app loop
    loop {
        clear_background(Color::from_rgba(5, 5, 20, 255));

        gargantua.draw();

        for ray in &mut rays {
            ray.step(&gargantua, 1.);
            ray.draw();
        }

        next_frame().await
    }
}
