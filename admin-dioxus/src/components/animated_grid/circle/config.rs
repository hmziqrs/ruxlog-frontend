use dioxus::prelude::*;
use dioxus_time::sleep;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use super::super::provider::{GridContext, GridData};
use super::state::{Direction, GridCircle, SpawnEdge};


pub const DEFAULT_CIRCLE_COUNT: usize = 1;
const SIDE_STEP_PERCENT: u8 = 20;
pub const DIAMETER_PX: f64 = 6.0;
pub const STEP_DURATION_MS: u32 = 1200;
pub const SCALE_DURATION_MS: u32 = 200;
const RESPAWN_DELAY_MS: u64 = 48;

pub static NEXT_CIRCLE_ID: AtomicU64 = AtomicU64::new(0);
static RNG_STATE: AtomicU64 = AtomicU64::new(0x9e3779b97f4a7c15);

pub type CircleSignal = Signal<GridCircle>;



pub fn circle_step(mut circle_sig: CircleSignal, grid_ctx: GridContext) {
    let grid = grid_ctx.grid_data.read().clone();

    if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
        return;
    }

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
            // At goal edge - scale out and fade before respawning
            circle.scale = 3.0;
            circle.opacity = 0.0;
            circle.moving = true;
            // ontransitionend will handle respawn after scale-out
        }
    }
}

fn decide_next_move(circle: &GridCircle, grid: &GridData) -> Option<(i32, i32, bool)> {
    let (dc, dr) = circle.travel_dir.delta();
    let mut target = (circle.col + dc, circle.row + dr);
    let mut did_side_step = false;

    // Only allow side-step if:
    // 1. We didn't just side-step on previous turn
    // 2. We haven't reached the goal edge
    // 3. Random chance triggers
    if !circle.just_side_stepped
        && !is_at_goal_edge(circle, grid)
        && random_chance(SIDE_STEP_PERCENT)
    {
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
    state.opacity = 0.0;
}

pub fn schedule_post_respawn(mut circle_sig: CircleSignal, _grid_ctx: GridContext) {
    spawn({
        async move {
            sleep(Duration::from_millis(RESPAWN_DELAY_MS)).await;
            let mut circle = circle_sig.write();
            circle.respawning = false;
            circle.scale = 1.0;
            circle.opacity = 1.0;
            // ontransitionend will trigger next step when scale-in completes
        }
    });
}

pub fn handle_transition_end(mut circle_sig: CircleSignal, grid_ctx: GridContext) {
    let (is_respawning, is_scale_out, is_scale_in, is_movement) = {
        let grid = grid_ctx.grid_data.read();
        let circle = circle_sig.read();
        (
            circle.respawning,
            circle.is_scale_out_complete(),
            circle.is_scale_in_complete(&grid),
            circle.is_movement_complete(),
        )
    };

    if is_respawning {
        return; // Ignore transitions during instant position changes
    }

    if is_scale_out {
        // Just finished scaling out at goal edge → respawn
        {
            let mut circle = circle_sig.write();
            circle.moving = false;
        }

        {
            let grid = grid_ctx.grid_data.read();
            let mut circle = circle_sig.write();
            respawn_circle_state(&mut circle, &grid);
        }

        schedule_post_respawn(circle_sig, grid_ctx);
    } else if is_scale_in {
        // Just finished scaling in after spawn → start moving
        circle_step(circle_sig, grid_ctx);
    } else if is_movement {
        // Just finished moving to next cell → continue
        {
            let mut circle = circle_sig.write();
            circle.moving = false;
        }
        circle_step(circle_sig, grid_ctx);
    }
}

pub fn spawn_circle_state(id: u64, grid: &GridData) -> GridCircle {
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
        opacity: 0.0,
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

pub fn random_u64() -> u64 {
    RNG_STATE
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |prev| {
            Some(prev.wrapping_mul(6364136223846793005).wrapping_add(1))
        })
        .unwrap()
}
