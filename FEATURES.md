# FEATURES

## Core Simulation
- [x] GRID-BASED PHYSICS - 2D grid simulation with cell-based materials.
- [x] MATERIAL BEHAVIORS - Static, Solid, Granular, Liquid, Heavy, Acid, Burning.
- [x] BEHAVIOR DISPATCH - Central dispatch system for different material behaviors.
- [x] TICK SYSTEM - Fixed timestep simulation at 60 ticks/second.
- [x] DIRECTION ALTERNATION - Alternating left/right movement prevents bias.
- [ ] TEMPERATURE SYSTEM - Heat propagation and thermal expansion.
- [ ] PRESSURE SYSTEM - Fluid pressure simulation for gases.
- [ ] ELECTRICITY - Conductive materials and spark propagation.

## Physics & Interactions
- [x] DENSITY INTERACTIONS - Materials sink/float based on density values.
- [x] GRAVITY - Downward acceleration for falling materials.
- [x] HORIZONTAL FLOW - Liquid spreading behavior.
- [x] CORROSION - Acid dissolves granular, liquid, and heavy materials.
- [x] COLLISION DETECTION - Cell occupancy prevents overlap.
- [ ] CHEMICAL REACTIONS - Material transformation on contact.
- [ ] EXPLOSIVE CHAIN REACTIONS - Fire spreading to flammable materials.
- [ ] VISCOSITY - Different flow rates for different liquids.

## Materials & Properties
- [x] SAND - Granular behavior with piling.
- [x] WATER - Liquid behavior with spreading.
- [x] STONE - Heavy falling material.
- [x] WALL - Immovable solid barrier.
- [x] ACID - Corrosive liquid material.
- [x] AIR - Eraser tool (empty space).
- [ ] OIL - Low-density flammable liquid.
- [ ] WOOD - Flammable solid material.
- [ ] METAL - Conductive solid material.
- [ ] LAVA - Hot liquid that ignites materials.
- [ ] ICE - Melts near heat sources.
- [ ] STEAM - Rises upward, condenses.

## Rendering & Visualization
- [x] RAYLIB RENDERING - Hardware-accelerated 2D rendering.
- [x] REAL-TIME DISPLAY - 60 FPS rendering loop.
- [x] CELL RENDERING - Color-coded material display.
- [x] FPS COUNTER - Frame rate overlay.
- [ ] PARTICLE EFFECTS - Sprite-based particle visuals.
- [ ] METADATA OVERLAY - Material count, simulation speed.
- [ ] ZOOM/PAN - Camera controls for large grids.
- [ ] DEBUG VISUALIZATION - Show behavior types, densities.

## User Interface
- [x] MATERIAL PALETTE - Sidebar with clickable material buttons.
- [x] ACTION BUTTONS - Play/Pause and Clear buttons.
- [x] BRUSH SYSTEM - Variable brush size for painting.
- [x] HOVER STATES - Visual feedback on interactive elements.
- [ ] HOTKEYS - Keyboard shortcuts for materials.
- [ ] TOOL TIPS - Hover help text for materials.
- [ ] CONTEXT MENU - Right-click for material info.
- [ ] SAVE/LOAD - Save simulation state to file.

## Input & Interaction
- [x] MOUSE PAINTING - Click to place materials.
- [x] BRUSH SIZING - Adjustable brush size 1-10.
- [x] PAUSE/PLAY - Toggle simulation state.
- [x] CLEAR GRID - Reset all cells to empty.
- [ ] LINE TOOL - Draw straight lines of material.
- [ ] RECTANGLE TOOL - Draw filled rectangles.
- [ ] CIRCLE TOOL - Draw filled circles.
- [ ] SPRAY TOOL - Randomized placement pattern.

## Configuration System
- [x] TOML CONFIG FILES - Separate configs for materials, menu, options.
- [x] MATERIAL DEFINITIONS - ID, name, color, behavior, density.
- [x] SIMULATION SETTINGS - Grid dimensions, cell size, FPS limit.
- [x] UI CONFIGURATION - Button labels, actions, colors.
- [ ] RELOAD CONFIG - Hot-reload without restart.
- [ ] CUSTOM BEHAVIORS - User-defined behavior scripts.
- [ ] PRESET MATERIALS - Save/load material presets.

## Performance & Optimization
- [x] RELEASE PROFILES - Optimized release builds with LTO.
- [x] DEPENDENCY OPTIMIZATION - Dependencies compiled at opt-level 3.
- [x] FIXED TIMESTEP - Decoupled simulation from rendering.
- [ ] MULTI-THREADING - Parallel behavior computation.
- [ ] CHUNK UPDATES - Only update active regions.
- [ ] GPU COMPUTE - Shader-based simulation.
- [ ] SPARSE GRID - Only store non-empty cells.

## Data Structures
- [x] FLAT VECTOR GRID - Vec<u32> for cell storage.
- [x] MATERIAL REGISTRY - O(n) lookup by ID.
- [x] BEHAVIOR ENUM - Match-based dispatch.
- [ ] SPATIAL HASH - Fast neighbor queries.
- [ ] QUADTREE - Region-based subdivision.
- [ ] ENTITY COMPONENT SYSTEM - ECS architecture.

## Persistence & Save System
- [ ] SAVE STATE - Serialize grid to binary format.
- [ ] LOAD STATE - Deserialize grid from file.
- [ ] AUTO-SAVE - Periodic backup to prevent data loss.
- [ ] SCREENSHOT - Export current state as image.
- [ ] REPLAY SYSTEM - Record and playback simulation.

## Extended Behaviors
- [ ] BURNING - Fire spreading and material ignition.
- [ ] GASEOUS - Upward floating and dispersal.
- [ ] EXPLOSIVE - Detonation with shockwave.
- [ ] FREEZING - Ice formation from water.
- [ ] MELTING - Solid to liquid transition.
- [ ] EVAPORATION - Liquid to gas transition.
- [ ] CONDENSATION - Gas to liquid transition.

## Advanced Materials
- [ ] PLANT - Grows upward, flammable.
- [ ] GUNPOWDER - Explosive powder.
- [ ] ACID GAS - Corrosive vapor.
- [ ] SALT - Dissolves in water, crystals form.
- [ ] MAGNET - Attracts metallic materials.
- [ ] PORTAL - Teleports passing materials.
- [ ] CLONE - Duplicates materials passing through.

## Developer Tools
- [ ] DEBUG MODE - Extended logging and visualization.
- [ ] PROFILING - Performance metrics overlay.
- [ ] INSPECTOR - Click cell to see properties.
- [ ] BEHAVIOR EDITOR - Visual behavior editor.
- [ ] MATERIAL EDITOR - Create materials in GUI.
- [ ] UNIT TESTS - Comprehensive test suite.

## Platform & Deployment
- [ ] WASM BUILD - Browser-based simulation.
- [ ] CROSS-PLATFORM - Windows, macOS, Linux support.
- [ ] ANDROID/iOS - Mobile touch controls.
- [ ] NATIVE INSTALLERS - Package for distribution.

## Observability
- [ ] STRUCTURED LOGGING - Using `tracing` crate.
- [ ] PERFORMANCE METRICS - Tick time, render time tracking.
- [ ] ERROR REPORTING - Graceful error handling.
- [ ] SIMULATION STATS - Material counts, updates per second.