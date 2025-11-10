# Animated Grid Circles – Implementation Plan

Goal: Replace keyframe-based single circle with a pool of independently animated circles that move one grid cell at a time using CSS transitions. Each circle owns its movement loop and respawn logic, reacts to grid changes, and never leaves bounds.

## Objectives
- Render many circles via a parent `AnimatedGridCircles` that holds `Vec<GlobalSignal<GridCircle>>` and maps each to `AnimatedGridCircle`.
- No CSS keyframes. Use `transition` on `transform` and update inline `style` per step; chain steps with `ontransitionend`.
- Circles spawn on an edge and travel toward the opposite direction (`travel_dir = spawn_edge.reverse()`).
- Move exactly one grid cell per transition. At each cell boundary: 20% chance to insert a vertical/horizontal “side step” while preserving the overall goal direction.
- Stay within bounds; clamp indices and respawn when exiting the playable area.
- Each circle handles its own `ontransitionend`, next move decision, and respawn.

## Data Model
- `enum Direction { Left, Right, Up, Down }`
  - `fn reverse(&self) -> Direction`
  - `fn axis(&self) -> Axis` where `Axis` = `Horizontal | Vertical`
  - `fn delta(&self) -> (i32, i32)`  // column/row delta for one step

- `enum SpawnEdge { Left, Right, Top, Bottom }`
  - `impl From<SpawnEdge> for Direction` → returns travel_dir as reverse(edge)

- `struct GridCircle {
    id: u64,
    col: i32,
    row: i32,
    travel_dir: Direction,
    pending_turn: Option<Direction>,    // optional side step to chain next
    moving: bool,                       // suppress overlapping transitions
    step_ms: u32,                       // duration per cell
    diameter_px: f64,
    spawn_edge: SpawnEdge,
    alive: bool,
}`

- `type CircleSig = GlobalSignal<GridCircle>`

## Integration Points
- Grid context: `GridData { vertical_lines, horizontal_lines, dimensions }` already exists (`src/components/animated_grid/provider.rs`).
  - Compute `cols = vertical_lines.len() - 1`, `rows = horizontal_lines.len() - 1`.
  - Compute `cell_w = vertical_lines.get(1).unwrap_or(&width).min(width) - vertical_lines[0]`, similarly `cell_h` from horizontal lines. With square grid, `cell_w ≈ cell_h`.
  - Convert `(col,row)` → `(x,y)` pixels:
    - `x = vertical_lines[col as usize]`
    - `y = horizontal_lines[row as usize]`
    - Center the circle: add half of cell size minus half of diameter.

## Rendering Structure
- Parent: `AnimatedGridCircles { count: usize }`
  - On mount or when grid is ready: create `count` circles with random edges and positions along that edge; set `travel_dir = edge.reverse()`.
  - Keep `Vec<CircleSig>` in component state and render: `{circles.iter().map(|c| rsx!(AnimatedGridCircle { circle: c.clone() }))}`
  - On grid change (dimensions/lines changed): notify circles to snap to nearest valid index and continue. Easiest: mark `moving = false` and force reflow to new `(x,y)`, then resume next step.

- Child: `AnimatedGridCircle { circle: CircleSig }`
  - Renders a small absolutely-positioned div:
    - `class: "absolute pointer-events-none will-change-transform transition-transform"`
    - `style: format!("transform: translate({x}px, {y}px); transition-duration: {}ms; width: {}px; height: {}px; border-radius: 9999px;", step_ms, d, d)`
  - Event: `ontransitionend: move |_| circle_step(circle.clone(), grid_ctx.clone())`
  - On first mount: kick off initial step by calling `circle_step` if not moving.

## Movement Algorithm (per circle)
1. Preconditions: require grid lines present (non-empty). If grid isn’t ready, delay/retry.
2. If `moving == true`, ignore requests to step.
3. Determine next move:
   - Primary move is one cell in `travel_dir` (`delta_col, delta_row`).
   - If `pending_turn.is_some()`, the next move is that direction; otherwise roll a 20% chance when the axis is horizontal (for horizontal travel) or vertical (for vertical travel) to insert a perpendicular side step:
     - Horizontal travel (Left/Right): optional `Up` or `Down` by 1.
     - Vertical travel (Up/Down): optional `Left` or `Right` by 1.
     - If chosen side step would exit bounds, discard it.
   - If we insert a side step, we set `pending_turn = Some(side_dir)` and do that move first. After finishing, clear `pending_turn` and schedule the primary move on the following transition end.
4. Bounds check: ensure `0 <= next_col <= cols` and `0 <= next_row <= rows`. If the primary move would exit bounds, mark `alive = false` and respawn.
5. Update state: set `moving = true`, update `col/row` to next indices, compute new `(x,y)` pixels, and update style. The CSS transition will animate from current transform to the new transform.
6. `ontransitionend` handler:
   - If `pending_turn` is set, immediately call `circle_step` again to execute the primary move.
   - Else, schedule the next step (e.g., via `spawn(async move { sleep(Duration::from_millis(0)).await; circle_step(..) })`) to keep the loop going without blocking.
   - Always set `moving = false` before deciding the next move.

## Respawn Logic
- When a circle attempts to move out of bounds:
  - Pick a new `spawn_edge` (random among 4), set `(col,row)` to a valid index on that edge (random along axis), set `travel_dir = edge.reverse()`.
  - Reset `pending_turn = None`, `moving = false`, and set transform immediately to spawn position with no transition (e.g., temporarily set `transition-duration: 0ms;`, then restore).
  - After re-positioning, trigger the first step in the new direction.

## Randomness Strategy
- Abstraction: `trait Rng { fn chance(percent: u8) -> bool; fn range(max: i32) -> i32; }`
- Web implementation: use `js_sys::Math::random()` to avoid extra deps, or add `getrandom`/`rand` with `wasm_js` feature.
- Deterministic fallback: simple xorshift/LCG seeded by `id` if needed.

## Durations and Easing
- Use `transition: transform linear;` with `transition-duration: step_ms ms` (e.g., 300–600ms). Keep per-circle `step_ms` to add variety.
- Keep sizes small (e.g., 6–8px) and include a soft shadow to match current visuals.

## API Sketch
- `AnimatedGridCircles { count: usize }`
- `AnimatedGridCircle { circle: GlobalSignal<GridCircle> }`
- Helpers:
  - `fn spawn_circle(id: u64, grid: &GridData) -> GridCircle`
  - `fn step_once(circle: &mut GridCircle, grid: &GridData, rng: &impl Rng) -> Option<(col,row)>` (returns next target or `None` for respawn)
  - `fn indices_to_px(col,row, grid: &GridData, d: f64) -> (f64,f64)`

## Edge Cases & Safeguards
- If grid changes mid-transition, finish current step then snap to nearest valid indices and continue; if indices out of range, respawn.
- Prevent back-to-back `ontransitionend` loops by checking `moving` and ensuring target changed.
- Clamp side steps that would leave bounds; don’t set `pending_turn` if it’s invalid.

## Work Plan
1. Types and utilities
   - Add `Direction`, `SpawnEdge`, `GridCircle`, RNG abstraction.
2. Parent container
   - Implement `AnimatedGridCircles` to create and render `count` circles from signals.
3. Child component
   - Implement `AnimatedGridCircle` with CSS transform transitions and `ontransitionend` chaining.
4. Grid integration
   - Read `GridData`, compute cell size and bounds, and convert indices to pixels.
5. Respawn + randomness
   - Implement edge spawn and 20% side-step logic with bounds checks.
6. Resize behavior
   - Watch grid changes, snap or respawn circles cleanly.
7. Polish
   - Tweak durations, sizes, and shadow to match visuals; add comments and docs.

## Testing/Verification
- Manual: resize window, confirm circles continue moving; watch edge respawns; verify side steps occur infrequently and never leave bounds.
- Logging hooks behind a feature flag for debugging transitions and decisions.

## Files to Add/Change (proposal)
- `src/components/animated_grid/circles.rs` (parent + child components)
- `src/components/animated_grid/types.rs` (Direction, SpawnEdge, GridCircle)
- Integrate from `src/components/animated_grid/mod.rs`

