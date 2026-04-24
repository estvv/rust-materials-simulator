# rust-materials-simulator

A grid-based particle simulation in Rust with physics-based material behaviors, density interactions, and real-time rendering.

[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## Architecture

```
┌─────────────────────────────────────────────┐
│            Render Window (Raylib)           │
│         Grid Display + UI Sidebar           │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│           AppState                          │
│  ├─ selected_material: u32                  │
│  ├─ paused: bool                            │
│  ├─ mouse_pos: (i32, i32)                   │
│  ├─ brush_size: u32                         │
│  └─ update_accumulator: f32                 │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│          World (Grid Simulation)            │
│  ├─ width/height: u32                       │
│  ├─ cells: Vec<u32> (material IDs)          │
│  ├─ tick: u32                               │
│  └─ update() with behavior dispatch         │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│      MaterialRegistry                       │
│  ├─ materials: Vec<Material>                │
│  ├─ get_behavior(id) -> Behavior            │
│  ├─ get_density(id) -> f32                  │
│  └─ get_color(id) -> &str                   │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│          Behavior Types                     │
│  ├─ Static (immovable)                      │
│  ├─ Solid (immovable)                       │
│  ├─ Granular (sand-like, falls)             │
│  ├─ Liquid (flows, spreads)                 │
│  ├─ Heavy (sinks through liquids)           │
│  ├─ Acid (corrodes materials)               │
│  └─ Burning (burns materials)               │
└─────────────────────────────────────────────┘
```

## Physics Simulation

**Density-Based Interactions:**

```
Sand   (1.5) ─┐
              ├─> Density comparison causes sinking/floating
Water (0.998)─┘

Stone (2.5) sinks through water
Wood  (0.5) would float on water
```

**Corrosion Mechanics:**

```
Acid + Sand  ──> Both destroyed
Acid + Water ──> Both destroyed
Acid + Stone ──> Both destroyed
Acid + Wall  ──> Wall remains
```

## Features

### Core Simulation
- **GRID-BASED PHYSICS** - 200x150 cell simulation grid
- **MATERIAL BEHAVIORS** - Static, Solid, Granular, Liquid, Heavy, Acid, Burning
- **DENSITY INTERACTIONS** - Materials sink/float based on density
- **TICK SYSTEM** - 60 updates per second fixed timestep

### Rendering & UI
- **REAL-TIME RENDERING** - Raylib-powered visualization
- **MATERIAL PALETTE** - Clickable material selector sidebar
- **BRUSH SYSTEM** - Variable brush size (1-10) for painting
- **FPS COUNTER** - Performance monitoring overlay

### Configuration
- **TOML CONFIGS** - Materials, menu, and options configured via TOML
- **CUSTOMIZABLE GRID** - Adjustable grid dimensions and cell size
- **FPS LIMITING** - Configurable framerate cap

See [FEATURES.md](FEATURES.md) for the full roadmap with planned features.

## Quick Start

```bash
# Build and run
cargo run

# Run with optimizations
cargo run --release
```

The simulation window opens with a material palette on the right side.

## Materials

| Material | Density | Behavior | Description |
|----------|---------|----------|-------------|
| Air | 0.0 | Solid | Empty space (eraser) |
| Sand | 1.5 | Granular | Falls down, piles up |
| Water | 0.998 | Liquid | Flows and spreads |
| Stone | 2.5 | Heavy | Sinks through liquids |
| Wall | 2.5 | Solid | Immovable barrier |
| Acid | 1.1 | Acid | Corrodes other materials |

## Controls

| Action | Control |
|--------|---------|
| Paint material | Left-click on grid |
| Select material | Left-click on palette |
| Pause/Play | Click "Play" button |
| Clear grid | Click "Clear" button |
| Brush size | Scroll wheel |

## Modules

| Module | Description |
|--------|-------------|
| `main.rs` | Application entry point, configuration loading |
| `core/world.rs` | Grid simulation with behavior dispatch |
| `core/material.rs` | Material definition struct |
| `core/behavior.rs` | Behavior enum (Static, Granular, Liquid, etc.) |
| `core/registry.rs` | Material registry and lookup |
| `core/state.rs` | Application state (selection, pause, brush) |
| `core/behaviors/` | Individual behavior implementations |
| `render/window.rs` | Window creation and main loop |
| `render/ui.rs` | Sidebar and button rendering |
| `config.rs` | TOML configuration loading |

## Behaviors

### Granular (Sand-like)
- Falls straight down if space available
- Falls diagonally if blocked below
- Piles up naturally
- Alternates left/right direction each tick

### Liquid (Water)
- Falls down if possible
- Spreads horizontally when blocked
- Fills containers
- Displaces lower-density materials

### Heavy (Stone)
- Falls through liquids
- Sinks based on density comparison
- Settles on granular materials

### Acid
- Falls like liquid
- Corrodes adjacent materials
- Both acid and target are destroyed
- Cannot dissolve walls

### Static/Solid
- Never moves
- Used for barriers and boundaries

## Configuration

### materials.toml
```toml
[[material]]
id = 1
name = "Sand"
color = "#EEDD82"
behavior = "granular"
density = 1.5
```

### options.toml
```toml
[simulation]
grid_width = 200
grid_height = 150
cell_size = 4
fps_limit = 60

[interaction]
brush_size = 3
default_material = 0
```

### menu.toml
```toml
[[buttons]]
id = "play_pause"
label = "Play"
action = "toggle_simulation"
color = "#4CAF50"
```

## Examples

### Basic Usage
```rust
use dynamic_materials::core::world::World;
use dynamic_materials::core::registry::MaterialRegistry;

let mut world = World::new(200, 150);
let mut registry = MaterialRegistry::new();

// Place sand
world.set_cell(100, 10, 1);

// Place water
world.set_cell(100, 20, 2);

// Simulate
world.update(&registry);
```

### Custom Material
```rust
// Add to materials.toml
[[material]]
id = 6
name = "Oil"
color = "#333333"
behavior = "liquid"
density = 0.8  // Floats on water!
```

## Project Structure

```
src/
├── main.rs                 # Entry point, config loading
├── config.rs               # TOML config parsing
├── core.rs                 # Core module exports
├── render.rs               # Render module exports
├── utils.rs                # Utility functions
├── core/
│   ├── world.rs            # Grid simulation logic
│   ├── material.rs         # Material definition
│   ├── behavior.rs         # Behavior enum
│   ├── registry.rs         # Material registry
│   ├── state.rs            # App state management
│   ├── cell.rs             # Cell utilities
│   └── behaviors/          # Behavior implementations
│       ├── mod.rs          # Behavior dispatcher
│       ├── granular.rs     # Granular behavior
│       ├── liquid.rs       # Liquid behavior
│       ├── heavy.rs        # Heavy behavior
│       ├── acid.rs         # Acid behavior
│       └── static_behavior.rs  # Static behavior
├── render/
│   ├── window.rs           # Window and main loop
│   ├── ui.rs               # UI components
│   └── ui/                 # UI subcomponents
└── utils/                  # Utility modules
    ├── position.rs         # Position utilities
    ├── color.rs            # Color utilities
    ├── size.rs             # Size utilities
    └── layout.rs           # Layout utilities

config/
├── materials.toml          # Material definitions
├── menu.toml               # UI buttons
└── options.toml            # Simulation settings
```

## Dependencies

| Crate | Version | Usage |
|-------|---------|-------|
| `raylib` | 5.5.1 | Window management and rendering |
| `serde` | 1.0 | Serialization |
| `toml` | 1.1.2 | Configuration parsing |
| `rand` | 0.8 | Random number generation |
| `log` | 0.4.29 | Logging |
| `env_logger` | 0.11.10 | Logging implementation |

## Simulation Algorithm

**Update Cycle:**

1. **Behavior Dispatch**
   - Iterate cells bottom-to-top
   - Alternate left-right direction each tick
   - Query behavior from registry
   - Execute behavior logic

2. **Density Resolution**
   - Compare densities of stacked materials
   - Swap positions if heavier above lighter
   - Only movable materials participate

3. **Corrosion Check**
   - Check acid neighbors (4-directional)
   - Dissolve dissolvable materials
   - Both cells become empty

4. **Tick Increment**
   - Used for alternating directions
   - Wrapping counter (u32::MAX)

---

Inspired by falling sand games and particle simulation systems.

## Roadmap

See [FEATURES.md](FEATURES.md) for planned and completed features.

## License

Licensed under [MIT](LICENSE)
