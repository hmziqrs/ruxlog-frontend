use dioxus::prelude::*;

use crate::components::sonner::{use_sonner, ToastOptions, SonnerToaster};
use crate::components::sonner::types::{ToasterProps, Position, Offset};

#[component]
pub fn SonnerDemoScreen() -> Element {
    // Controls for provider defaults (Phase 5 QA)
    let mut position = use_signal(|| Position::BottomCenter);
    let mut expand = use_signal(|| false);
    let mut visible = use_signal(|| 3usize);
    let mut offset_str = use_signal(|| "24px".to_string());
    let mut mobile_offset_str = use_signal(|| "16px".to_string());
    let mut breakpoint = use_signal(|| 640i32);

    // Build defaults from controls
    let defaults = {
        let mut d = ToasterProps::default();
        d.position = position();
        d.expand = expand();
        d.visible_toasts = visible();
        d.offset = Offset::Text(offset_str());
        d.mobile_offset = Offset::Text(mobile_offset_str());
        d.mobile_breakpoint_px = breakpoint();
        d
    };

    rsx! {
        div { class: "p-6 space-y-4",
            h2 { class: "text-xl font-semibold", "Sonner Demo" }

            // Controls
            div { class: "flex flex-wrap gap-2 items-center",
                div { class: "font-medium mr-2", "Position:" }
                for (label, pos) in [
                    ("Top Left", Position::TopLeft),
                    ("Top Center", Position::TopCenter),
                    ("Top Right", Position::TopRight),
                    ("Bottom Left", Position::BottomLeft),
                    ("Bottom Center", Position::BottomCenter),
                    ("Bottom Right", Position::BottomRight),
                ] {
                    button { 
                        class: { if position() == pos { 
                            "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                        } else { 
                            "px-2 py-1 rounded border hover:bg-accent text-sm"
                        }},
                        onclick: move |_| position.set(pos),
                        {label}
                    }
                }
            }
            div { class: "flex flex-wrap gap-2 items-center",
                div { class: "font-medium mr-2", "Offset:" }
                for label in ["16px", "24px", "32px", "48px"] {
                    button { 
                        class: { if offset_str() == label { 
                            "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                        } else { 
                            "px-2 py-1 rounded border hover:bg-accent text-sm"
                        }},
                        onclick: move |_| offset_str.set(label.to_string()),
                        {label}
                    }
                }
                div { class: "font-medium ml-4 mr-2", "Mobile Offset:" }
                for label in ["8px", "12px", "16px", "24px"] {
                    button { 
                        class: { if mobile_offset_str() == label { 
                            "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                        } else { 
                            "px-2 py-1 rounded border hover:bg-accent text-sm"
                        }},
                        onclick: move |_| mobile_offset_str.set(label.to_string()),
                        {label}
                    }
                }
                div { class: "font-medium ml-4 mr-2", "Breakpoint:" }
                for (label, bp) in [("640", 640), ("768", 768), ("1024", 1024)] {
                    button { 
                        class: { if breakpoint() == bp { 
                            "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                        } else { 
                            "px-2 py-1 rounded border hover:bg-accent text-sm"
                        }},
                        onclick: move |_| breakpoint.set(bp),
                        {label}
                    }
                }
            }
            div { class: "flex flex-wrap gap-2 items-center",
                div { class: "font-medium mr-2", "Visible:" }
                for n in [1usize, 2, 3, 4, 5] {
                    button { 
                        class: { if visible() == n { 
                            "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                        } else { 
                            "px-2 py-1 rounded border hover:bg-accent text-sm"
                        }},
                        onclick: move |_| visible.set(n),
                        {format!("{}", n)}
                    }
                }
                div { class: "font-medium ml-4 mr-2", "Expand:" }
                button { 
                    class: { if !expand() { 
                        "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                    } else { 
                        "px-2 py-1 rounded border hover:bg-accent text-sm"
                    }},
                    onclick: move |_| expand.set(false),
                    "Off"
                }
                button { 
                    class: { if expand() { 
                        "px-2 py-1 rounded border text-sm bg-primary text-primary-foreground"
                    } else { 
                        "px-2 py-1 rounded border hover:bg-accent text-sm"
                    }},
                    onclick: move |_| expand.set(true),
                    "On"
                }
            }

            // Nested provider with adjustable defaults so hooks below use it
            SonnerToaster { defaults: defaults,
                DemoContent {}
            }

            p { class: "text-sm text-muted-foreground", "Hover to pause auto-dismiss; change position and offsets to QA stacking/offset behavior." }
        }
    }
}

#[component]
fn DemoContent() -> Element {
    let sonner = use_sonner();

    
    rsx! {
        div { class: "space-x-2",
            button {
                class: "px-3 py-2 rounded-md bg-green-600 text-white hover:bg-green-700",
                onclick: move |_| {
                    let mut opts = ToastOptions::default();
                    opts.duration_ms = Some(9000);
                    // let sonner_clone = sonner;
                    // opts.on_auto_close = Some(Callback::new(move |id| {
                    //     sonner_clone.info(format!("Success auto-closed: {id}"), ToastOptions::default());
                    // }));
                    sonner.success("Saved successfully".to_string(), opts);
                },
                "Show Success"
            }
            button {
                class: "px-3 py-2 rounded-md bg-red-600 text-white hover:bg-red-700",
                onclick: move |_| {
                    let mut opts = ToastOptions::default();
                    opts.duration_ms = Some(2000);
                    let sonner_clone = sonner;
                    opts.on_auto_close = Some(Callback::new(move |id| {
                        sonner_clone.info(format!("Error auto-closed: {id}"), ToastOptions::default());
                    }));
                    sonner.error("Something went wrong".to_string(), opts);
                },
                "Show Error"
            }
        }
    }
}
