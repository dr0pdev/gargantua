use ::glam::Vec2;
use macroquad::prelude::*;

// window size
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// scientific constants
const G: f32 = 6.6743e-11;
const SPEED_OF_LIGHT: f32 = 299792458.0;
const SLOWDOWN: f32 = 1000000000.;

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
struct Ray {
    x: f32,
    y: f32,
    direction: Vec2,
    trail: Vec<Vec2>,
}

impl Ray {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, 10., 10., WHITE);

        for i in 1..self.trail.len() {
            println!("{}", (i / self.trail.len()));
            draw_rectangle(
                self.trail[i].x,
                self.trail[i].y,
                5.,
                5.,
                Color {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: (i / self.trail.len()) as f32,
                },
            );
        }
    }

    fn push_back(&mut self) {
        self.trail.push(Vec2 {
            x: self.x,
            y: self.y,
        })
    }

    fn step(&mut self) {
        self.x += self.direction.x * SPEED_OF_LIGHT / SLOWDOWN;
        self.y += self.direction.y * SPEED_OF_LIGHT / SLOWDOWN;
        self.push_back();
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

    let mut rays: Vec<Ray> = vec![Ray {
        x: 100.,
        y: 10. + ((HEIGHT / 2) as f32),
        direction: Vec2 { x: 1., y: 0. },
        trail: vec![],
    }];
    // get the schwarzschild radius of the black hole
    gargantua.calc_r_s();
    println!("{}", gargantua.r_s);

    // main app loop
    loop {
        clear_background(BLACK);
        gargantua.draw();
        for ray in &mut rays {
            ray.draw();
            ray.step();
            ray.push_back();
            // println!("{:?}", ray.trail);
        }
        next_frame().await
    }
}
