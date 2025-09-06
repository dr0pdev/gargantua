# Gargantua - Black Hole Gravitational Lensing Simulation

A real-time black hole visualization that demonstrates gravitational lensing using Einstein's General Relativity equations. Built with Rust and WebAssembly for high-performance physics simulation.

## Features

- **Real Physics**: Implements actual Schwarzschild geodesics (not approximations!)
- **Parallel Processing**: Uses Rayon for multi-core ray calculations
- **Real-time Rendering**: Smooth 60fps simulation with beautiful visual effects
- **WebAssembly**: Runs in browsers with near-native performance
- **Interactive Controls**: Adjust black hole mass and ray count in real-time

## Quick Start

### Native Desktop Version
```bash
cargo run
```

### Web Version (WebAssembly)
```bash
# Build the WebAssembly package
wasm-pack build --target web --out-dir pkg

# Serve the files (choose one):
python3 -m http.server 8000
# or
npx serve .
# or
cargo install basic-http-server && basic-http-server

# Open http://localhost:8000 in your browser
```

## Physics Implementation

The simulation uses the Schwarzschild metric to calculate null geodesics (light ray paths) around a black hole:

- **Schwarzschild Radius**: `r_s = 2GM/c²`
- **Geodesic Equations**: Full relativistic ray tracing
- **Gravitational Lensing**: Realistic light bending effects
- **Event Horizon**: Rays disappear when crossing the Schwarzschild radius

## Controls

- **Mass Slider**: Adjust black hole mass (1-10 × 10²⁸ kg)
- **Ray Count**: Control number of light rays (10-100)
- **Reset**: Restart the simulation
- **Pause/Resume**: Control simulation playback

## Technical Details

### Dependencies
- **Rust**: Systems programming language
- **Macroquad**: Cross-platform game framework
- **Rayon**: Data parallelism library
- **Glam**: Linear algebra library
- **WebAssembly**: Browser deployment target

### Performance
- **Parallel Physics**: Multi-core ray calculations
- **Optimized Rendering**: Efficient trail and particle systems
- **WebAssembly**: Near-native browser performance

## Visual Effects

- **Accretion Disk**: Orange-to-red gradient representing superheated matter
- **Event Horizon**: Pure black center with bright orange outline
- **Light Rays**: White photon trails showing gravitational lensing
- **Trail System**: Persistent ray paths for beautiful light bending visualization

## Educational Value

This simulation demonstrates:
- **General Relativity**: How mass curves spacetime
- **Gravitational Lensing**: How black holes bend light
- **Event Horizon**: The point of no return
- **Accretion Disks**: Superheated matter around black holes

## Development

### Building for Web
```bash
# Install WebAssembly target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack

# Build WebAssembly package
wasm-pack build --target web --out-dir pkg
```

### Customization
- Modify `simulation.rs` for physics parameters
- Adjust `index.html` for web interface changes
- Update `main.rs` for desktop version features

## License

MIT License - feel free to use for educational and commercial purposes.

---

*"The most incomprehensible thing about the universe is that it is comprehensible."* - Albert Einstein
