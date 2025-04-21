use dioxus::prelude::*;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div { class: "container mx-auto p-6 min-h-screen transition-colors duration-300",
            // Header
            div { class: "mb-8 flex flex-col md:flex-row md:items-center md:justify-between gap-2",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                        "Blog Dashboard"
                    }
                    p { class: "text-zinc-600 dark:text-zinc-400",
                        "Manage your content and track performance"
                    }
                }
                button { class: "btn btn-primary", onclick: move |_| {}, "Create New Post" }
            }

            // Stats Cards
            div { class: "grid grid-cols-1 md:grid-cols-4 gap-6 mb-8",
                StatCard {
                    title: "Total Posts",
                    value: "120",
                    change: "+4 this week",
                    icon: "📝",
                }
                StatCard {
                    title: "Total Views",
                    value: "45.2K",
                    change: "+12% this month",
                    icon: "👁️",
                }
                StatCard {
                    title: "Comments",
                    value: "892",
                    change: "+89 this week",
                    icon: "💬",
                }
                StatCard {
                    title: "Users",
                    value: "2.3K",
                    change: "+105 this month",
                    icon: "👤",
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-8",
                MiniStatCard {
                    title: "Revenue",
                    value: "$12,400",
                    icon: "💰",
                    color: "emerald-500",
                }
                MiniStatCard {
                    title: "Active Users",
                    value: "1,230",
                    icon: "🟢",
                    color: "sky-500",
                }
                MiniStatCard {
                    title: "Bounce Rate",
                    value: "32%",
                    icon: "↩️",
                    color: "rose-500",
                }
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-8",
                ProgressCard {
                    title: "Server Uptime",
                    percent: 99,
                    color: "bg-green-500",
                }
                ProgressCard {
                    title: "Storage Used",
                    percent: 68,
                    color: "bg-yellow-500",
                }
            }
            div { class: "mb-8",
                NotificationCard {
                    title: "New User",
                    message: "Alice joined your platform.",
                    icon: "👤",
                }
                NotificationCard {
                    title: "System Update",
                    message: "A new update is available.",
                    icon: "⬆️",
                }
                NotificationCard {
                    title: "Warning",
                    message: "Storage is almost full.",
                    icon: "⚠️",
                }
            }

            // Analytics/Charts (placeholder)
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8",
                div { class: "bg-zinc-200/40 dark:bg-zinc-900/60 border  rounded-xl p-6 shadow flex flex-col justify-center items-center min-h-[260px] w-full",
                    h2 { class: "text-lg font-semibold text-zinc-800 dark:text-zinc-100 mb-2",
                        "Page Views (Chart)"
                    }
                    p { class: "text-zinc-500 dark:text-zinc-400 text-sm mb-4", "[Chart placeholder]" }
                    div { class: "w-full flex flex-col gap-2 mt-2",
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "Today" }
                            span { "+1,200" }
                        }
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "This Week" }
                            span { "+8,900" }
                        }
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "This Month" }
                            span { "+45,200" }
                        }
                    }
                }
                div { class: "bg-zinc-200/40 dark:bg-zinc-900/60 border  rounded-xl p-6 shadow flex flex-col justify-center items-center min-h-[260px] w-full",
                    h2 { class: "text-lg font-semibold text-zinc-800 dark:text-zinc-100 mb-2",
                        "Posts by Category (Chart)"
                    }
                    p { class: "text-zinc-500 dark:text-zinc-400 text-sm mb-4", "[Chart placeholder]" }
                    div { class: "w-full flex flex-col gap-2 mt-2",
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "Tech" }
                            span { "45 posts" }
                        }
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "Lifestyle" }
                            span { "30 posts" }
                        }
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "Travel" }
                            span { "25 posts" }
                        }
                        div { class: "flex justify-between text-xs text-zinc-600 dark:text-zinc-400",
                            span { "Food" }
                            span { "20 posts" }
                        }
                    }
                }
            }

            // Recent Posts
            div { class: "bg-zinc-200/40 dark:bg-zinc-900/60 border  p-6 rounded-xl mb-8",
                div { class: "flex justify-between items-center mb-4",
                    h2 { class: "text-xl font-semibold text-zinc-800 dark:text-zinc-100",
                        "Recent Posts"
                    }
                    button { class: "text-zinc-500 hover:text-zinc-800 dark:text-zinc-400 dark:hover:text-zinc-100 text-sm transition-colors",
                        "View All"
                    }
                }
                table { class: "min-w-full text-sm text-zinc-700 dark:text-zinc-300",
                    thead {
                        tr {
                            th { class: "px-4 py-2 text-left font-semibold", "Post" }
                            th { class: "px-4 py-2 text-left font-semibold", "Author" }
                            th { class: "px-4 py-2 text-left font-semibold", "Status" }
                            th { class: "px-4 py-2 text-left font-semibold", "Views" }
                            th { class: "px-4 py-2 text-left font-semibold", "Published" }
                        }
                    }
                    tbody {
                        RecentPostRow {
                            title: "Getting Started with Next.js 13",
                            author: "John Doe",
                            status: "Published",
                            views: "1234",
                            published: "2024-03-10",
                        }
                        RecentPostRow {
                            title: "The Future of AI in 2024",
                            author: "Jane Smith",
                            status: "Draft",
                            views: "0",
                            published: "-",
                        }
                        RecentPostRow {
                            title: "Ultimate Guide to TypeScript",
                            author: "Mike Johnson",
                            status: "Published",
                            views: "892",
                            published: "2024-03-08",
                        }
                        RecentPostRow {
                            title: "Modern CSS Techniques",
                            author: "Sarah Wilson",
                            status: "Under Review",
                            views: "0",
                            published: "-",
                        }
                        RecentPostRow {
                            title: "How to use Dioxus with Tailwind",
                            author: "Alex Kim",
                            status: "Published",
                            views: "432",
                            published: "2025-04-10",
                        }
                        RecentPostRow {
                            title: "Deploying Rust Web Apps",
                            author: "Samira Patel",
                            status: "Draft",
                            views: "0",
                            published: "-",
                        }
                    }
                }
            }

            // Recent Comments
            div { class: "bg-zinc-200/40 dark:bg-zinc-900/60 border  p-6 rounded-xl",
                div { class: "flex justify-between items-center mb-4",
                    h2 { class: "text-xl font-semibold text-zinc-800 dark:text-zinc-100",
                        "Recent Comments"
                    }
                    button { class: "text-zinc-500 hover:text-zinc-800 dark:text-zinc-400 dark:hover:text-zinc-100 text-sm transition-colors",
                        "View All"
                    }
                }
                div { class: "space-y-4",
                    RecentCommentRow {
                        user: "Alex Thompson",
                        comment: "Great article! Very informative.",
                        post: "Getting Started with Next.js 13",
                        time: "5 minutes ago",
                    }
                    RecentCommentRow {
                        user: "Maria Garcia",
                        comment: "Would love to see more examples on this topic.",
                        post: "Ultimate Guide to TypeScript",
                        time: "15 minutes ago",
                    }
                    RecentCommentRow {
                        user: "David Chen",
                        comment: "Thanks for sharing these insights!",
                        post: "The Future of AI in 2024",
                        time: "1 hour ago",
                    }
                    RecentCommentRow {
                        user: "Priya Singh",
                        comment: "This dashboard looks great!",
                        post: "How to use Dioxus with Tailwind",
                        time: "2 hours ago",
                    }
                    RecentCommentRow {
                        user: "Liam O'Brien",
                        comment: "Looking forward to more Rust content.",
                        post: "Deploying Rust Web Apps",
                        time: "yesterday",
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, change: &'static str, icon: &'static str) -> Element {
    rsx! {
        div { class: "bg-zinc-200/40 dark:bg-zinc-900/60 border  p-6 rounded-xl flex flex-col gap-2 shadow",
            div { class: "flex items-center justify-between",
                span { class: "text-2xl", "{icon}" }
                span {
                    class: format!(
                        "text-sm {}",
                        if change.contains('+') { "text-green-500" } else { "text-red-500" },
                    ),
                    "{change}"
                }
            }
            h3 { class: "text-zinc-600 dark:text-zinc-400 text-sm font-medium mt-2",
                "{title}"
            }
            p { class: "text-2xl font-bold text-zinc-900 dark:text-zinc-100 mt-1",
                "{value}"
            }
        }
    }
}

#[component]
fn RecentPostRow(title: &'static str, author: &'static str, status: &'static str, views: &'static str, published: &'static str) -> Element {
    let (status_bg, status_fg) = match status {
        "Published" => ("bg-green-900/50", "text-green-400"),
        "Draft" => ("bg-zinc-800", "text-zinc-400"),
        _ => ("bg-yellow-900/50", "text-yellow-400"),
    };
    rsx! {
        tr { class: "hover:bg-zinc-300/30 dark:hover:bg-zinc-800/50 transition-colors",
            td { class: "px-4 py-2 font-medium", "{title}" }
            td { class: "px-4 py-2", "{author}" }
            td { class: "px-4 py-2",
                span {
                    class: format!(
                        "px-2 inline-flex text-xs leading-5 font-semibold rounded-full {} {}",
                        status_bg,
                        status_fg,
                    ),
                    "{status}"
                }
            }
            td { class: "px-4 py-2", "{views}" }
            td { class: "px-4 py-2", "{published}" }
        }
    }
}

#[component]
fn RecentCommentRow(user: &'static str, comment: &'static str, post: &'static str, time: &'static str) -> Element {
    rsx! {
        div { class: "flex items-start space-x-4 border-b  pb-4 last:border-b-0",
            div { class: "h-10 w-10 rounded-full bg-zinc-400 dark:bg-zinc-700 flex items-center justify-center text-zinc-100 font-bold" }
            div { class: "flex-1",
                div { class: "flex items-center justify-between",
                    h3 { class: "text-sm font-medium text-zinc-800 dark:text-zinc-100",
                        "{user}"
                    }
                    p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{time}" }
                }
                p { class: "text-xs text-zinc-400", "on {post}" }
                p { class: "mt-1 text-sm text-zinc-700 dark:text-zinc-300", "{comment}" }
            }
        }
    }
}

#[component]
fn MiniStatCard(title: &'static str, value: &'static str, icon: &'static str, color: &'static str) -> Element {
    rsx! {
        div {
            class: format!(
                "flex flex-col items-start justify-between p-4 rounded-lg shadow bg-{} text-white min-w-[160px]",
                color,
            ),
            div { class: "flex items-center gap-2 mb-2",
                span { class: "text-xl", "{icon}" }
                h4 { class: "text-sm font-semibold", "{title}" }
            }
            p { class: "text-2xl font-bold", "{value}" }
        }
    }
}

#[component]
fn ProgressCard(title: &'static str, percent: u8, color: &'static str) -> Element {
    rsx! {
        div { class: "p-4 rounded-lg shadow bg-zinc-200/40 dark:bg-zinc-900/60 border  flex flex-col gap-2 min-w-[180px]",
            h4 { class: "text-sm font-semibold text-zinc-700 dark:text-zinc-200", "{title}" }
            div { class: "w-full bg-zinc-300 dark:bg-zinc-800 rounded-full h-2 overflow-hidden",
                div {
                    class: format!("h-2 rounded-full {}", color),
                    style: format!("width: {}%", percent),
                }
            }
            p { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-1", "{percent}% complete" }
        }
    }
}

#[component]
fn NotificationCard(title: &'static str, message: &'static str, icon: &'static str) -> Element {
    rsx! {
        div { class: "flex items-start gap-3 p-4 rounded-lg bg-blue-100 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-800 mb-2",
            span { class: "text-xl text-blue-500 dark:text-blue-300", "{icon}" }
            div {
                h5 { class: "font-semibold text-sm text-blue-900 dark:text-blue-100",
                    "{title}"
                }
                p { class: "text-xs text-blue-800 dark:text-blue-200", "{message}" }
            }
        }
    }
}
