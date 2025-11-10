use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

use crate::hooks::use_effect_cleanup;
use getrandom::fill;

const NUM_LINES: usize = 12;
const DOT_COUNT: usize = 8;
const MIN_DURATION: f32 = 6.0;
const MAX_DURATION: f32 = 11.5;
const MAX_DELAY: f32 = 3.5;

const GRID_KEYFRAMES: &str = r#"
    @keyframes loginGridMoveX {
        from { left: -32px; opacity: 0; }
        10% { opacity: 1; }
        90% { opacity: 1; }
        to { left: calc(100vw + 32px); opacity: 0; }
    }
    @keyframes loginGridMoveY {
        from { top: -32px; opacity: 0; }
        10% { opacity: 1; }
        90% { opacity: 1; }
        to { top: calc(100vh + 32px); opacity: 0; }
    }
"#;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
struct AnimatedDot {
    id: Uuid,
    orientation: Orientation,
    line_index: usize,
    duration: f32,
    delay: f32,
}

fn random_u32() -> u32 {
    let mut bytes = [0u8; 4];
    if fill(&mut bytes).is_err() {
        return 0;
    }
    u32::from_ne_bytes(bytes)
}

fn random_bool() -> bool {
    random_u32() & 1 == 0
}

fn random_range_f32(min: f32, max: f32) -> f32 {
    let fraction = random_u32() as f32 / u32::MAX as f32;
    min + (max - min) * fraction
}

fn random_index(max_inclusive: usize) -> usize {
    if max_inclusive == usize::MAX {
        return random_u32() as usize;
    }
    let range = max_inclusive + 1;
    (random_u32() as usize) % range
}

impl AnimatedDot {
    fn random() -> Self {
        let orientation = if random_bool() {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        };

        Self {
            id: Uuid::new_v4(),
            orientation,
            line_index: random_index(NUM_LINES),
            duration: random_range_f32(MIN_DURATION, MAX_DURATION),
            delay: random_range_f32(0.0, MAX_DELAY),
        }
    }

    fn line_position(&self) -> f64 {
        (self.line_index as f64 * 100.0) / NUM_LINES as f64
    }

    fn total_duration_ms(&self) -> u32 {
        ((self.duration + self.delay).max(0.5) * 1000.0) as u32
    }
}

fn schedule_dot_respawn(
    dots: Signal<Vec<AnimatedDot>>,
    lifecycle: Arc<AtomicBool>,
    dot_id: Uuid,
    lifetime_ms: u32,
) {
    let mut dots_for_spawn = dots.clone();
    let lifecycle_for_spawn = lifecycle.clone();

    spawn(async move {
        TimeoutFuture::new(lifetime_ms).await;

        if !lifecycle_for_spawn.load(Ordering::Relaxed) {
            return;
        }

        let new_dot = AnimatedDot::random();
        let should_continue = {
            let mut current = dots_for_spawn.write();
            if let Some(slot) = current.iter_mut().find(|dot| dot.id == dot_id) {
                *slot = new_dot.clone();
                true
            } else {
                false
            }
        };

        if should_continue {
            schedule_dot_respawn(
                dots_for_spawn,
                lifecycle_for_spawn,
                new_dot.id,
                new_dot.total_duration_ms(),
            );
        }
    });
}

#[component]
pub fn AnimatedGridBackground() -> Element {
    let dots = use_signal(|| {
        (0..DOT_COUNT)
            .map(|_| AnimatedDot::random())
            .collect::<Vec<_>>()
    });
    let lifecycle = use_signal(|| Arc::new(AtomicBool::new(true)));
    let initialized = use_signal(|| false);

    {
        let dots_for_effect = dots.clone();
        let lifecycle_for_effect = lifecycle();
        let mut init = initialized.clone();

        use_effect(move || {
            if init() {
                return;
            }

            init.set(true);
            for dot in dots_for_effect.read().iter().cloned() {
                schedule_dot_respawn(
                    dots_for_effect.clone(),
                    lifecycle_for_effect.clone(),
                    dot.id,
                    dot.total_duration_ms(),
                );
            }
        });
    }

    {
        let lifecycle_for_cleanup = lifecycle();
        use_effect_cleanup(move || {
            lifecycle_for_cleanup.store(false, Ordering::Relaxed);
        });
    }

    let line_positions: Vec<f64> = (0..=NUM_LINES)
        .map(|i| i as f64 * 100.0 / NUM_LINES as f64)
        .collect();
    let active_dots = dots();

    rsx! {
        Fragment {
            style {
                dangerous_inner_html: GRID_KEYFRAMES,
            }
            div {
                class: "pointer-events-none absolute inset-0 -z-10 bg-transparent",
                aria_hidden: "true",

                {line_positions.iter().map(|pos| {
                    let offset = format!("{pos:.4}%");
                    rsx! {
                        div {
                            class: "absolute inset-y-0 border-l border-border",
                            style: format!("left: {offset}; opacity: 0.15;"),
                        }
                    }
                })},

                {line_positions.iter().map(|pos| {
                    let offset = format!("{pos:.4}%");
                    rsx! {
                        div {
                            class: "absolute inset-x-0 border-t border-border",
                            style: format!("top: {offset}; opacity: 0.15;"),
                        }
                    }
                })},

                {active_dots.iter().map(|dot| {
                    let pos = format!("{:.4}%", dot.line_position());
                    let (pos_style, animation_name) = match dot.orientation {
                        Orientation::Horizontal => (format!("top: {pos};"), "loginGridMoveX"),
                        Orientation::Vertical => (format!("left: {pos};"), "loginGridMoveY"),
                    };

                    rsx! {
                        div {
                            key: "{dot.id}",
                            class: "absolute w-2 h-2 rounded-full bg-primary",
                            style: format!(
                                "{pos_style} animation: {animation} {duration}s linear forwards; animation-delay: {delay}s; opacity: 0.85;",
                                pos_style = pos_style,
                                animation = animation_name,
                                duration = dot.duration,
                                delay = dot.delay,
                            ),
                        }
                    }
                })},
            }
        }
    }
}
