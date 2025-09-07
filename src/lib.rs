use wasm_bindgen::prelude::*;
use web_sys::console;
use std::sync::Mutex;

// Import the main simulation code
mod simulation;
use simulation::{Simulation, BlackHole};

// Global simulation state
static SIMULATION: Mutex<Option<Simulation>> = Mutex::new(None);

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() {
    log!("Gargantua WebAssembly module loaded!");
}

// Export a function to initialize the simulation
#[wasm_bindgen]
pub fn init_simulation(width: i32, height: i32) {
    log!("Initializing simulation with size {}x{}", width, height);
    let sim = Simulation::new(width, height);
    *SIMULATION.lock().unwrap() = Some(sim);
}

// Export a function to update the simulation
#[wasm_bindgen]
pub fn update_simulation() {
    if let Ok(mut sim_guard) = SIMULATION.lock() {
        if let Some(ref mut sim) = *sim_guard {
            sim.update();
        }
    }
}

// Export a function to get ray positions
#[wasm_bindgen]
pub fn get_ray_positions() -> Vec<f64> {
    if let Ok(sim_guard) = SIMULATION.lock() {
        if let Some(ref sim) = *sim_guard {
            let mut positions = Vec::new();
            for ray in &sim.rays {
                positions.push(ray.x);
                positions.push(ray.y);
                positions.push(if ray.disabled { 0.0 } else { 1.0 });
            }
            return positions;
        }
    }
    Vec::new()
}

// Export a function to get black hole position
#[wasm_bindgen]
pub fn get_black_hole_position() -> Vec<f64> {
    if let Ok(sim_guard) = SIMULATION.lock() {
        if let Some(ref sim) = *sim_guard {
            return vec![
                sim.black_hole.position.x as f64,
                sim.black_hole.position.y as f64,
                sim.black_hole.r_s
            ];
        }
    }
    vec![400.0, 400.0, 20.0] // Default values
}

// Export a function to update black hole mass
#[wasm_bindgen]
pub fn update_black_hole_mass(mass: f64) {
    log!("Updating black hole mass to: {} kg", mass);
    if let Ok(mut sim_guard) = SIMULATION.lock() {
        if let Some(ref mut sim) = *sim_guard {
            sim.black_hole.mass = mass;
            sim.black_hole.calc_r_s();
        }
    }
}

// Export a function to reset the simulation
#[wasm_bindgen]
pub fn reset_simulation() {
    log!("Resetting simulation...");
    if let Ok(mut sim_guard) = SIMULATION.lock() {
        if let Some(ref mut sim) = *sim_guard {
            // Reset all rays to their initial positions
            let num_rays = sim.rays.len();
            let start_x = 50.0;
            let start_y = 50.0;
            
            for (i, ray) in sim.rays.iter_mut().enumerate() {
                // Reset to same starting position (matching main.rs approach)
                ray.x = start_x;
                ray.y = start_y;
                ray.disabled = false;
                ray.trail.clear();
                
                // Create rays with slightly different angles for spread pattern (matching main.rs)
                let angle_offset = (i as f64 - num_rays as f64 / 2.0) * 0.05; // Small angle variations
                let velocity_x = 10.0 * angle_offset.cos() + 10.;
                let velocity_y = 10.0 * angle_offset.sin() + 10.;
                
                ray.initialize_velocity(&sim.black_hole, glam::Vec2::new(velocity_x as f32, velocity_y as f32));
            }
        }
    }
}

// Export a function to get current ray count
#[wasm_bindgen]
pub fn get_ray_count() -> usize {
    if let Ok(sim_guard) = SIMULATION.lock() {
        if let Some(ref sim) = *sim_guard {
            return sim.rays.len();
        }
    }
    50
}

// Export a function to update ray count
#[wasm_bindgen]
pub fn update_ray_count(new_count: usize) {
    log!("Updating ray count to: {}", new_count);
    if let Ok(mut sim_guard) = SIMULATION.lock() {
        if let Some(ref mut sim) = *sim_guard {
            let current_count = sim.rays.len();
            
            if new_count > current_count {
                // Add more rays
                let start_x = 50.0;
                let start_y = 50.0;
                
                for i in current_count..new_count {
                    let mut ray = simulation::Ray::new(start_x, start_y);
                    
                    // Create rays with slightly different angles for spread pattern (matching main.rs)
                    let angle_offset = (i as f64 - new_count as f64 / 2.0) * 0.05; // Small angle variations
                    let velocity_x = 10.0 * angle_offset.cos() + 10.;
                    let velocity_y = 10.0 * angle_offset.sin() + 10.;
                    
                    ray.initialize_velocity(&sim.black_hole, glam::Vec2::new(velocity_x as f32, velocity_y as f32));
                    sim.rays.push(ray);
                }
            } else if new_count < current_count {
                // Remove rays
                sim.rays.truncate(new_count);
            }
        }
    }
}

// Export a function to get simulation info
#[wasm_bindgen]
pub fn get_simulation_info() -> String {
    format!("Gargantua Black Hole Simulation v0.1.0\nSchwarzschild Radius: {} meters", 
            simulation::calculate_schwarzschild_radius())
}

// Debug function to get initial ray positions
#[wasm_bindgen]
pub fn get_initial_ray_positions() -> Vec<f64> {
    if let Ok(sim_guard) = SIMULATION.lock() {
        if let Some(ref sim) = *sim_guard {
            let mut positions = Vec::new();
            for ray in &sim.rays {
                positions.push(ray.x);
                positions.push(ray.y);
                positions.push(ray.dr);
                positions.push(ray.dphi);
            }
            return positions;
        }
    }
    Vec::new()
}
