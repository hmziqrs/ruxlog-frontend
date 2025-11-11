use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use dioxus::prelude::*;
use dioxus_time::sleep;

use super::provider::{use_grid_context, GridContext, GridData};

const DEFAULT_CIRCLE_COUNT: usize = 16;
const SIDE_STEP_PERCENT: u8 = 20;
const STEP_DURATION_RANGE: (u32, u32) = (320, 640);
const DIAMETER_RANGE: (f64, f64) = (5.0, 8.0);
const RESPAWN_DELAY_MS: u64 = 48;

static NEXT_CIRCLE_ID: AtomicU64 = AtomicU64::new(0);
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9e3779b97f4a7c15);

type CirclesSignal = Signal<Vec<GridCircle>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn perpendicular(self) -> [Direction; 2] {
        match self {
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpawnEdge {
    Left,
    Right,
    Top,
    Bottom,
}

impl SpawnEdge {
    fn random() -> Self {
        match random_usize(4) {
            0 => SpawnEdge::Left,
            1 => SpawnEdge::Right,
            2 => SpawnEdge::Top,
            _ => SpawnEdge::Bottom,
        }
    }
}

impl From<SpawnEdge> for Direction {
    fn from(edge: SpawnEdge) -> Self {
        match edge {
            SpawnEdge::Left => Direction::Right,
            SpawnEdge::Right => Direction::Left,
            SpawnEdge::Top => Direction::Down,
            SpawnEdge::Bottom => Direction::Up,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct GridCircle {
    id: u64,
    col: i32,
    row: i32,
    travel_dir: Direction,
    moving: bool,
    respawning: bool,
    step_ms: u32,
    diameter_px: f64,
    spawn_edge: SpawnEdge,
    alive: bool,
}

#[derive(Clone, Debug)]
struct GridMetrics {
    cols: i32,
    rows: i32,
}

impl GridMetrics {
    fn from(grid: &GridData) -> Option<Self> {
        if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
            return None;
        }
        if grid.vertical_lines[1] - grid.vertical_lines[0] <= 0.0
            || grid.horizontal_lines[1] - grid.horizontal_lines[0] <= 0.0
        {
            return None;
        }

        Some(Self {
            cols: (grid.vertical_lines.len() - 1) as i32,
            rows: (grid.horizontal_lines.len() - 1) as i32,
        })
    }

    fn in_bounds(&self, col: i32, row: i32) -> bool {
        col >= 0 && row >= 0 && col < self.cols && row < self.rows
    }
}

#[component]
pub fn AnimatedGridCircles(#[props(optional)] count: Option<usize>) -> Element {
    let ctx = use_grid_context();
    let circle_count = count.unwrap_or(DEFAULT_CIRCLE_COUNT);
    let circles: CirclesSignal = use_signal(|| Vec::new());

    use_effect({
        let ctx = ctx.clone();
        let circles_signal = circles.clone();
        move || {
            let mut circles = circles_signal;
            let grid = ctx.grid_data.read().clone();
            let Some(metrics) = GridMetrics::from(&grid) else {
                return;
            };

            {
                let mut stored = circles.write();
                if stored.len() > circle_count {
                    stored.truncate(circle_count);
                }

                while stored.len() < circle_count {
                    let id = NEXT_CIRCLE_ID.fetch_add(1, Ordering::Relaxed);
                    stored.push(spawn_circle_state(id, &metrics));
                }
            }

            let len = circles.read().len();
            for index in 0..len {
                enforce_circle_bounds(index, circles, ctx.clone(), &metrics);
                circle_step(index, circles, ctx.clone());
            }
        }
    });

    let circles_snapshot = circles.read().clone();

    rsx! {
        div {
            class: "absolute inset-0 pointer-events-none",
            {circles_snapshot
                .into_iter()
                .enumerate()
                .map(|(index, circle_state)| {
                    rsx! {
                        AnimatedGridCircle {
                            key: "animated-circle-{circle_state.id}",
                            index,
                            circle: circle_state,
                            circles: circles.clone(),
                            grid_ctx: ctx.clone(),
                        }
                    }
                })}
        }
    }
}

#[derive(Props, Clone)]
struct AnimatedGridCircleProps {
    index: usize,
    circle: GridCircle,
    circles: CirclesSignal,
    grid_ctx: GridContext,
}

impl PartialEq for AnimatedGridCircleProps {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.circle == other.circle
    }
}

#[component]
fn AnimatedGridCircle(props: AnimatedGridCircleProps) -> Element {
    let AnimatedGridCircleProps {
        index,
        circle,
        circles,
        grid_ctx,
    } = props;

    let grid = grid_ctx.grid_data.read().clone();
    let Some((x, y)) = indices_to_px(circle.col, circle.row, &grid, circle.diameter_px) else {
        return rsx! {};
    };

    let transition_style = if circle.respawning {
        "transition: none;".to_string()
    } else {
        format!("transition: transform linear {}ms;", circle.step_ms)
    };

    let style = format!(
        "transform: translate({x:.2}px, {y:.2}px); width: {d:.2}px; height: {d:.2}px; border-radius: 9999px; {transition_style}",
        d = circle.diameter_px,
    );

    rsx! {
        div {
            class: "absolute will-change-transform pointer-events-none bg-primary shadow-[0_0_8px_1px_var(--primary)]",
            style: "{style}",
            ontransitionend: move |_| {
                handle_transition_end(index, circles.clone(), grid_ctx.clone());
            },
        }
    }
}

fn handle_transition_end(index: usize, mut circles: CirclesSignal, grid_ctx: GridContext) {
    {
        let mut all = circles.write();
        if let Some(circle) = all.get_mut(index) {
            circle.moving = false;
        } else {
            return;
        }
    }

    circle_step(index, circles, grid_ctx);
}

fn circle_step(index: usize, mut circles: CirclesSignal, grid_ctx: GridContext) {
    let grid = grid_ctx.grid_data.read().clone();
    let Some(metrics) = GridMetrics::from(&grid) else {
        return;
    };

    let mut schedule_respawn = false;

    {
        let mut all = circles.write();
        let Some(circle) = all.get_mut(index) else {
            return;
        };

        if circle.respawning || circle.moving {
            return;
        }

        if let Some((next_col, next_row)) = decide_next_move(circle, &metrics) {
            circle.col = next_col;
            circle.row = next_row;
            circle.moving = true;
        } else {
            respawn_circle_state(circle, &metrics);
            schedule_respawn = true;
        }
    }

    if schedule_respawn {
        schedule_post_respawn(index, circles, grid_ctx);
    }
}

fn decide_next_move(circle: &GridCircle, metrics: &GridMetrics) -> Option<(i32, i32)> {
    let (dc, dr) = circle.travel_dir.delta();
    let mut target = (circle.col + dc, circle.row + dr);

    if random_chance(SIDE_STEP_PERCENT) {
        let options = circle.travel_dir.perpendicular();
        let side_choice = if random_bool() {
            options[0]
        } else {
            options[1]
        };
        let (sc, sr) = side_choice.delta();
        let side_target = (circle.col + sc, circle.row + sr);
        if metrics.in_bounds(side_target.0, side_target.1) {
            target = side_target;
        }
    }

    if metrics.in_bounds(target.0, target.1) {
        Some(target)
    } else {
        None
    }
}

fn respawn_circle_state(state: &mut GridCircle, metrics: &GridMetrics) {
    let edge = SpawnEdge::random();
    let travel_dir: Direction = edge.into();
    let (col, row) = match edge {
        SpawnEdge::Left => (0, random_i32(metrics.rows)),
        SpawnEdge::Right => (metrics.cols - 1, random_i32(metrics.rows)),
        SpawnEdge::Top => (random_i32(metrics.cols), 0),
        SpawnEdge::Bottom => (random_i32(metrics.cols), metrics.rows - 1),
    };

    state.col = col.clamp(0, metrics.cols.saturating_sub(1));
    state.row = row.clamp(0, metrics.rows.saturating_sub(1));
    state.travel_dir = travel_dir;
    state.spawn_edge = edge;
    state.step_ms = random_step_duration();
    state.diameter_px = random_diameter();
    state.moving = false;
    state.respawning = true;
    state.alive = true;
}

fn enforce_circle_bounds(
    index: usize,
    mut circles: CirclesSignal,
    grid_ctx: GridContext,
    metrics: &GridMetrics,
) {
    let mut needs_respawn = false;

    {
        let mut all = circles.write();
        let Some(state) = all.get_mut(index) else {
            return;
        };

        if !metrics.in_bounds(state.col, state.row) {
            respawn_circle_state(state, metrics);
            needs_respawn = true;
        }
    }

    if needs_respawn {
        schedule_post_respawn(index, circles, grid_ctx);
    }
}

fn schedule_post_respawn(index: usize, mut circles: CirclesSignal, grid_ctx: GridContext) {
    spawn({
        async move {
            sleep(Duration::from_millis(RESPAWN_DELAY_MS)).await;
            {
                let mut all = circles.write();
                if let Some(circle) = all.get_mut(index) {
                    circle.respawning = false;
                } else {
                    return;
                }
            }
            circle_step(index, circles, grid_ctx);
        }
    });
}

fn spawn_circle_state(id: u64, metrics: &GridMetrics) -> GridCircle {
    let edge = SpawnEdge::random();
    let travel_dir: Direction = edge.into();
    let (col, row) = match edge {
        SpawnEdge::Left => (0, random_i32(metrics.rows)),
        SpawnEdge::Right => (metrics.cols - 1, random_i32(metrics.rows)),
        SpawnEdge::Top => (random_i32(metrics.cols), 0),
        SpawnEdge::Bottom => (random_i32(metrics.cols), metrics.rows - 1),
    };

    GridCircle {
        id,
        col: col.clamp(0, metrics.cols.saturating_sub(1)),
        row: row.clamp(0, metrics.rows.saturating_sub(1)),
        travel_dir,
        moving: false,
        respawning: false,
        step_ms: random_step_duration(),
        diameter_px: random_diameter(),
        spawn_edge: edge,
        alive: true,
    }
}

fn indices_to_px(col: i32, row: i32, grid: &GridData, diameter: f64) -> Option<(f64, f64)> {
    let col_idx = usize::try_from(col).ok()?;
    let row_idx = usize::try_from(row).ok()?;
    let x = *grid.vertical_lines.get(col_idx)?;
    let y = *grid.horizontal_lines.get(row_idx)?;

    let cell_w = if grid.vertical_lines.len() > 1 {
        grid.vertical_lines[1] - grid.vertical_lines[0]
    } else {
        0.0
    };
    let cell_h = if grid.horizontal_lines.len() > 1 {
        grid.horizontal_lines[1] - grid.horizontal_lines[0]
    } else {
        0.0
    };

    Some((x + (cell_w - diameter) / 2.0, y + (cell_h - diameter) / 2.0))
}

fn random_step_duration() -> u32 {
    random_between_u32(STEP_DURATION_RANGE.0, STEP_DURATION_RANGE.1)
}

fn random_diameter() -> f64 {
    random_between_f64(DIAMETER_RANGE.0, DIAMETER_RANGE.1)
}

fn random_between_u32(min: u32, max: u32) -> u32 {
    if max <= min {
        return min;
    }
    let spread = max - min;
    min + (random_u64() as u32 % (spread + 1))
}

fn random_between_f64(min: f64, max: f64) -> f64 {
    if max <= min {
        return min;
    }
    let ratio = (random_u64() >> 11) as f64 / ((1u64 << 53) as f64);
    min + (max - min) * ratio
}

fn random_i32(max: i32) -> i32 {
    if max <= 0 {
        return 0;
    }
    (random_u64() % max as u64) as i32
}

fn random_usize(max: usize) -> usize {
    if max == 0 {
        return 0;
    }
    (random_u64() % max as u64) as usize
}

fn random_bool() -> bool {
    random_u64() & 1 == 1
}

fn random_chance(percent: u8) -> bool {
    if percent >= 100 {
        return true;
    }
    if percent == 0 {
        return false;
    }
    (random_u64() % 100) < percent as u64
}

fn random_u64() -> u64 {
    RNG_STATE
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |prev| {
            Some(prev.wrapping_mul(6364136223846793005).wrapping_add(1))
        })
        .unwrap()
}
