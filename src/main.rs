use ::glam::Vec2;
use macroquad::prelude::*;

// window size
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// scientific constants
const G: f32 = 6.6743e-11;
const SPEED_OF_LIGHT: f32 = 299792458.0;
const SLOWDOWN: f32 = 90000000.;

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
    mass: f32,
    r_s: f32,
}

impl BlackHole {
    fn calc_r_s(&mut self) {
        self.r_s = (2.0 * G * self.mass) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.r_s, RED);
    }
}

// struct for Ray
#[derive(Default)]
struct Ray {
    x: f32,
    y: f32,
    r: f32,
    phi: f32,
    direction: Vec2,
    trail: Vec<Vec2>,
}

impl Ray {
    fn get_polar(&mut self, bh: &BlackHole) {
        self.r = ((self.x - bh.position.x).powi(2) + (self.y - bh.position.y).powi(2)).sqrt();
        self.phi = self.y.atan2(self.x);
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, 10., 10., WHITE);

        for i in 1..self.trail.len() {
            draw_rectangle(
                self.trail[i].x,
                self.trail[i].y,
                10.,
                10.,
                Color {
                    r: 120.,
                    g: 120.,
                    b: 120.,
                    a: 0.1,
                },
            );
        }
    }

    fn push_back(&mut self) {
        if (self.x as u32) % 3 == 0 {
            self.trail.push(Vec2 {
                x: self.x - 1.,
                y: self.y,
            })
        }
    }

    fn step(&mut self, bh: &BlackHole) {
        self.get_polar(bh);
        // if the ray is within the schwarzchild radius: STOP
        if self.r < bh.r_s {
            return;
        }
        self.x += self.direction.x * SPEED_OF_LIGHT / SLOWDOWN;
        self.y += self.direction.y * SPEED_OF_LIGHT / SLOWDOWN;
        self.push_back();

        // if self.trail.len() > 50 {
        //     self.trail.clear();
        // }
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut gargantua = BlackHole {
        position: Vec2 {
            x: 500.,
            y: (HEIGHT / 2) as f32,
        },
        mass: 6e28,
        r_s: 0.,
    };

    let mut rays: Vec<Ray> = vec![];

    for i in -1..11 {
        rays.push(Ray {
            x: 100.,
            y: (i as f32 * 20.) + ((HEIGHT / 3) as f32),
            direction: Vec2 { x: 1., y: 0. },
            ..Default::default()
        });
    }

    // get the schwarzschild radius of the black hole
    gargantua.calc_r_s();

    // main app loop
    loop {
        clear_background(BLACK);
        gargantua.draw();
        for ray in &mut rays {
            ray.draw();
            ray.step(&gargantua);
            ray.push_back();
        }
        next_frame().await
    }
}
