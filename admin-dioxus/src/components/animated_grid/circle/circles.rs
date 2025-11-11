use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use dioxus::prelude::*;
use dioxus_time::sleep;

use super::circle::AnimatedGridCircle;
use super::state::{Direction, GridCircle, SpawnEdge};
use crate::components::animated_grid::provider::{use_grid_context, GridContext, GridData};

const DEFAULT_CIRCLE_COUNT: usize = 16;
const SIDE_STEP_PERCENT: u8 = 20;
const STEP_DURATION_RANGE: (u32, u32) = (320, 640);
const DIAMETER_RANGE: (f64, f64) = (5.0, 8.0);
const RESPAWN_DELAY_MS: u64 = 48;

static NEXT_CIRCLE_ID: AtomicU64 = AtomicU64::new(0);
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9e3779b97f4a7c15);

pub type CirclesSignal = Signal<Vec<GridCircle>>;

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
            let grid = ctx.grid_data.peek().clone();

            if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
                return;
            }

            {
                let mut stored = circles.write();
                if stored.len() > circle_count {
                    stored.truncate(circle_count);
                }

                while stored.len() < circle_count {
                    let id = NEXT_CIRCLE_ID.fetch_add(1, Ordering::Relaxed);
                    stored.push(spawn_circle_state(id, &grid));
                }
            }

            let len = circles.read().len();
            for index in 0..len {
                enforce_circle_bounds(index, circles, ctx.clone(), &grid);
                circle_step(index, circles, ctx.clone());
            }
        }
    });

    rsx! {
        div {
            class: "absolute inset-0 pointer-events-none",
            {circles()
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

pub fn circle_step(index: usize, mut circles: CirclesSignal, grid_ctx: GridContext) {
    let grid = grid_ctx.grid_data.read().clone();

    if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
        return;
    }

    let mut schedule_respawn = false;

    {
        let mut all = circles.write();
        let Some(circle) = all.get_mut(index) else {
            return;
        };

        if circle.respawning || circle.moving {
            return;
        }

        if let Some((next_col, next_row)) = decide_next_move(circle, &grid) {
            circle.col = next_col;
            circle.row = next_row;
            circle.moving = true;
        } else {
            respawn_circle_state(circle, &grid);
            schedule_respawn = true;
        }
    }

    if schedule_respawn {
        schedule_post_respawn(index, circles, grid_ctx);
    }
}

fn decide_next_move(circle: &GridCircle, grid: &GridData) -> Option<(i32, i32)> {
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
        if grid.in_bounds(side_target.0, side_target.1) {
            target = side_target;
        }
    }

    if grid.in_bounds(target.0, target.1) {
        Some(target)
    } else {
        None
    }
}

fn respawn_circle_state(state: &mut GridCircle, grid: &GridData) {
    let edge = SpawnEdge::random();
    let travel_dir: Direction = edge.into();
    let (col, row) = match edge {
        SpawnEdge::Left => (0, random_i32(grid.rows())),
        SpawnEdge::Right => (grid.cols() - 1, random_i32(grid.rows())),
        SpawnEdge::Top => (random_i32(grid.cols()), 0),
        SpawnEdge::Bottom => (random_i32(grid.cols()), grid.rows() - 1),
    };

    state.col = col.clamp(0, grid.cols().saturating_sub(1));
    state.row = row.clamp(0, grid.rows().saturating_sub(1));
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
    grid: &GridData,
) {
    let mut needs_respawn = false;

    {
        let mut all = circles.write();
        let Some(state) = all.get_mut(index) else {
            return;
        };

        if !grid.in_bounds(state.col, state.row) {
            respawn_circle_state(state, grid);
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

fn spawn_circle_state(id: u64, grid: &GridData) -> GridCircle {
    let edge = SpawnEdge::random();
    let travel_dir: Direction = edge.into();
    let (col, row) = match edge {
        SpawnEdge::Left => (0, random_i32(grid.rows())),
        SpawnEdge::Right => (grid.cols() - 1, random_i32(grid.rows())),
        SpawnEdge::Top => (random_i32(grid.cols()), 0),
        SpawnEdge::Bottom => (random_i32(grid.cols()), grid.rows() - 1),
    };

    GridCircle {
        id,
        col: col.clamp(0, grid.cols().saturating_sub(1)),
        row: row.clamp(0, grid.rows().saturating_sub(1)),
        travel_dir,
        moving: false,
        respawning: false,
        step_ms: random_step_duration(),
        diameter_px: random_diameter(),
        spawn_edge: edge,
        alive: true,
    }
}

pub fn indices_to_px(col: i32, row: i32, grid: &GridData, diameter: f64) -> Option<(f64, f64)> {
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

pub fn random_usize(max: usize) -> usize {
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
