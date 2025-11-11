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
pub const DIAMETER_PX: f64 = 6.0;
pub const STEP_DURATION_MS: u32 = 1200;
const RESPAWN_DELAY_MS: u64 = 48;

static NEXT_CIRCLE_ID: AtomicU64 = AtomicU64::new(0);
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9e3779b97f4a7c15);

pub type CircleSignal = Signal<GridCircle>;

#[component]
pub fn AnimatedGridCircles(#[props(optional)] count: Option<usize>) -> Element {
    let grid_ctx = use_grid_context();
    let circle_count = count.unwrap_or(DEFAULT_CIRCLE_COUNT);

    let mut circles = use_signal(|| Vec::new());

    use_effect(move || {
        if !circles.read().is_empty() {
            return;
        }

        let grid = grid_ctx.grid_data.read();
        if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
            return;
        }

        let new_circles = (0..circle_count)
            .map(|_| {
                let id = NEXT_CIRCLE_ID.fetch_add(1, Ordering::Relaxed);
                Signal::new(spawn_circle_state(id, &grid))
            })
            .collect::<Vec<_>>();

        circles.set(new_circles);
    });

    use_effect(move || {
        let circle_list = circles.read();
        for circle_sig in circle_list.iter() {
            spawn({
                let circle_sig = *circle_sig;
                let grid_ctx = grid_ctx.clone();
                async move {
                    sleep(Duration::from_millis(random_u64() % 200)).await;
                    schedule_post_respawn(circle_sig, grid_ctx);
                }
            });
        }
    });

    rsx! {
        div {
            class: "absolute inset-0 pointer-events-none",
            {circles.read().iter().map(|circle_sig| {
                let id = circle_sig.read().id;
                rsx! {
                    AnimatedGridCircle {
                        key: "{id}",
                        circle: *circle_sig,
                    }
                }
            })}
        }
    }
}

pub fn circle_step(mut circle_sig: CircleSignal, grid_ctx: GridContext) {
    let grid = grid_ctx.grid_data.read().clone();

    if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
        return;
    }

    let mut scale_out = false;

    {
        let mut circle = circle_sig.write();

        if circle.respawning || circle.moving {
            return;
        }

        if let Some((next_col, next_row, did_side_step)) = decide_next_move(&circle, &grid) {
            circle.col = next_col;
            circle.row = next_row;
            circle.moving = true;
            circle.just_side_stepped = did_side_step;
        } else {
            // At goal edge - scale out before respawning
            circle.scale = 3.0;
            circle.moving = true;
            scale_out = true;
        }
    }

    if scale_out {
        schedule_post_scale_out(circle_sig, grid_ctx);
    }
}

fn schedule_post_scale_out(mut circle_sig: CircleSignal, grid_ctx: GridContext) {
    spawn({
        async move {
            // Wait for scale-out transition to complete
            sleep(Duration::from_millis(STEP_DURATION_MS as u64)).await;
            {
                let grid = grid_ctx.grid_data.read();
                let mut circle = circle_sig.write();
                respawn_circle_state(&mut circle, &grid);
            }
            schedule_post_respawn(circle_sig, grid_ctx);
        }
    });
}

fn decide_next_move(circle: &GridCircle, grid: &GridData) -> Option<(i32, i32, bool)> {
    let (dc, dr) = circle.travel_dir.delta();
    let mut target = (circle.col + dc, circle.row + dr);
    let mut did_side_step = false;

    // Only allow side-step if:
    // 1. We didn't just side-step on previous turn
    // 2. We haven't reached the goal edge
    // 3. Random chance triggers
    if !circle.just_side_stepped && !is_at_goal_edge(circle, grid) && random_chance(SIDE_STEP_PERCENT) {
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
            did_side_step = true;
        }
    }

    if grid.in_bounds(target.0, target.1) {
        Some((target.0, target.1, did_side_step))
    } else {
        None
    }
}

fn is_at_goal_edge(circle: &GridCircle, grid: &GridData) -> bool {
    match circle.spawn_edge {
        SpawnEdge::Left => circle.col >= grid.cols() - 1,
        SpawnEdge::Right => circle.col <= 0,
        SpawnEdge::Top => circle.row >= grid.rows() - 1,
        SpawnEdge::Bottom => circle.row <= 0,
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
    state.moving = false;
    state.respawning = true;
    state.alive = true;
    state.just_side_stepped = false;
    state.scale = 3.0;
}

fn schedule_post_respawn(mut circle_sig: CircleSignal, grid_ctx: GridContext) {
    spawn({
        async move {
            sleep(Duration::from_millis(RESPAWN_DELAY_MS)).await;
            {
                let mut circle = circle_sig.write();
                circle.respawning = false;
                circle.scale = 1.0;
            }
            // Wait for scale-in transition to complete, then start movement
            sleep(Duration::from_millis(STEP_DURATION_MS as u64)).await;
            circle_step(circle_sig, grid_ctx);
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
        respawning: true,
        spawn_edge: edge,
        alive: true,
        just_side_stepped: false,
        scale: 3.0,
    }
}

pub fn indices_to_px(col: i32, row: i32, grid: &GridData) -> Option<(f64, f64)> {
    let col_idx = usize::try_from(col).ok()?;
    let row_idx = usize::try_from(row).ok()?;
    let x = *grid.vertical_lines.get(col_idx)?;
    let y = *grid.horizontal_lines.get(row_idx)?;

    // Center circle on grid line intersection
    Some((x - DIAMETER_PX / 2.0, y - DIAMETER_PX / 2.0))
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
