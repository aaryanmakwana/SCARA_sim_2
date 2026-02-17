# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

SCARA robot simulator written in Rust using egui/eframe for GUI visualization. Simulates a 2-link planar SCARA arm with forward/inverse kinematics.

## Build & Run Commands

```bash
# Build the project
cargo build

# Run the simulator
cargo run

# Build release version
cargo build --release

# Run release version
cargo run --release

# Check for errors without building
cargo check

# Format code
cargo fmt

# Run clippy lints
cargo clippy
```

## Architecture

### Active Modules

- `main.rs` - Entry point; configures eframe window (1000x700) and launches `ScaraApp`
- `app.rs` - Main application struct implementing `eframe::App`
  - Left panel: target position sliders, robot geometry controls, simulation stats
  - Central panel: robot visualization with dynamic scaling
  - Simulation loop runs at fixed `dt` timestep
- `geometry/scara.rs` - Core robot model:
  - `Scara` struct: link lengths (l1, l2), base position
  - `ScaraState` struct: joint angles (theta1, theta2)
  - `fk()`: Forward kinematics returning joint1 and end-effector positions
  - `ik()`: Inverse kinematics with elbow-up/down configuration (controlled by `ELBOW_UP` constant)

### Scaffolded Modules (Empty)

The following modules exist as directory structure but are not yet implemented:
- `control/` - PID, feedforward, position loop
- `simulation/` - integrator, state, timebase
- `trajectory/` - s-curve, trapezoidal, time parameterization
- `motor/` - electrical, mechanical models
- `kinematics/` - coordinate transforms
- `planner/` - path planning, lookahead, feedrate
- `toolpath/` - line, circle, curvature primitives
- `metrics/` - tracking error, settling time, bandwidth profiling
- `ui/` - separate view components
- `common/` - shared types and math utilities

## Key Conventions

- Coordinates use `[f64; 2]` arrays for 2D positions
- Angles stored in radians
- World-to-screen transform in `app.rs::world_to_screen()` handles coordinate system conversion (Y-axis flipped)
- Workspace radius = l1 + l2; visualization scales dynamically to fit window
