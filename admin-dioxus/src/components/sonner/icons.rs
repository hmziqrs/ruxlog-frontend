//! Sonner Icons — minimal inline SVG set + loader bars
use dioxus::prelude::*;

fn class_attr(base: &str, extra: Option<&str>) -> String {
    if let Some(e) = extra { format!("{} {}", base, e) } else { base.to_string() }
}

pub fn icon_success(extra_class: Option<&str>) -> Element {
    let class = class_attr("sonner-icon h-5 w-5 text-foreground", extra_class);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: class,
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "M7 12l3 3 7-7", stroke_linecap: "round", stroke_linejoin: "round" }
        }
    }
}

pub fn icon_info(extra_class: Option<&str>) -> Element {
    let class = class_attr("sonner-icon h-5 w-5 text-foreground", extra_class);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: class,
            circle { cx: "12", cy: "12", r: "10" }
            line { x1: "12", y1: "8", x2: "12", y2: "8" }
            line { x1: "12", y1: "12", x2: "12", y2: "16" }
        }
    }
}

pub fn icon_warning(extra_class: Option<&str>) -> Element {
    let class = class_attr("sonner-icon h-5 w-5 text-foreground", extra_class);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: class,
            path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z", stroke_linecap: "round", stroke_linejoin: "round" }
            line { x1: "12", y1: "9", x2: "12", y2: "13" }
            line { x1: "12", y1: "17", x2: "12", y2: "17" }
        }
    }
}

pub fn icon_error(extra_class: Option<&str>) -> Element {
    let class = class_attr("sonner-icon h-5 w-5 text-foreground", extra_class);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: class,
            circle { cx: "12", cy: "12", r: "10" }
            line { x1: "15", y1: "9", x2: "9", y2: "15" }
            line { x1: "9", y1: "9", x2: "15", y2: "15" }
        }
    }
}

pub fn icon_close(extra_class: Option<&str>) -> Element {
    let class = class_attr("sonner-icon h-4 w-4", extra_class);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", class: class,
            line { x1: "18", y1: "6", x2: "6", y2: "18" }
            line { x1: "6", y1: "6", x2: "18", y2: "18" }
        }
    }
}

/// A simple spinner using Tailwind's animate-spin class.
pub fn loader_spinner(extra_class: Option<&str>) -> Element {
    let class = class_attr("sonner-loader h-5 w-5 animate-spin text-foreground", extra_class);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", class: class,
            circle { cx: "12", cy: "12", r: "10", stroke: "currentColor", opacity: "0.25", stroke_width: "4" }
            path { d: "M22 12a10 10 0 0 1-10 10", stroke: "currentColor", stroke_width: "4", stroke_linecap: "round" }
        }
    }
}
