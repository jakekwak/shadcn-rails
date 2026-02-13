use dioxus::prelude::*;
use crate::components::Badge;

#[component]
pub fn Feature17() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mb-12 flex max-w-3xl flex-col gap-4",
                    Badge { variant: "secondary", "Features" }
                    h2 {
                        class: "text-3xl font-medium md:text-4xl lg:text-5xl",
                        "Fully featured components for Shadcn UI & Tailwind"
                    }
                }
                div {
                    class: "grid gap-12 md:grid-cols-2",
                    // Quality
                    div {
                        class: "flex gap-6 space-y-4 rounded-lg md:block",
                        span {
                            class: "bg-accent flex size-10 shrink-0 items-center justify-center rounded-full md:size-12",
                            // GitPullRequest
                            svg { class: "size-4 md:size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                circle { cx: "18", cy: "18", r: "3" }
                                circle { cx: "6", cy: "6", r: "3" }
                                path { d: "M13 6h3a2 2 0 0 1 2 2v7" }
                                path { d: "M6 9v12" }
                            }
                        }
                        div {
                            h3 { class: "font-medium md:mb-2 md:text-xl", "Quality" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Built with attention to detail and best practices. Every component is thoroughly tested and follows modern React patterns for reliability and performance." }
                        }
                    }
                    // Experience
                    div {
                        class: "flex gap-6 space-y-4 rounded-lg md:block",
                        span {
                            class: "bg-accent flex size-10 shrink-0 items-center justify-center rounded-full md:size-12",
                            // SquareKanban
                            svg { class: "size-4 md:size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
                                path { d: "M8 7v7" }
                                path { d: "M12 7v4" }
                                path { d: "M16 7v9" }
                            }
                        }
                        div {
                            h3 { class: "font-medium md:mb-2 md:text-xl", "Experience" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Crafted with user experience in mind. Each component is designed to be intuitive, accessible, and provide smooth interactions across all devices." }
                        }
                    }
                    // Support
                    div {
                        class: "flex gap-6 space-y-4 rounded-lg md:block",
                        span {
                            class: "bg-accent flex size-10 shrink-0 items-center justify-center rounded-full md:size-12",
                            // RadioTower
                            svg { class: "size-4 md:size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M4.9 16.1C1 12.2 1 5.8 4.9 1.9" }
                                path { d: "M7.8 4.7a6.14 6.14 0 0 0-.8 7.5" }
                                circle { cx: "12", cy: "9", r: "2" }
                                path { d: "M16.2 4.7a6.14 6.14 0 0 1 .8 7.5" }
                                path { d: "M19.1 1.9a10.56 10.56 0 0 1 0 14.2" }
                                path { d: "M12 11v9" }
                            }
                        }
                        div {
                            h3 { class: "font-medium md:mb-2 md:text-xl", "Support" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Comprehensive documentation and community support. Get help when you need it with detailed guides, examples, and active community assistance." }
                        }
                    }
                    // Innovation
                    div {
                        class: "flex gap-6 space-y-4 rounded-lg md:block",
                        span {
                            class: "bg-accent flex size-10 shrink-0 items-center justify-center rounded-full md:size-12",
                            // WandSparkles
                            svg { class: "size-4 md:size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72" }
                                path { d: "m14 7 3 3" }
                                path { d: "M5 6v4" }
                                path { d: "M19 14v4" }
                                path { d: "M10 2v2" }
                                path { d: "M7 8H3" }
                                path { d: "M21 16h-4" }
                                path { d: "M11 3H9" }
                            }
                        }
                        div {
                            h3 { class: "font-medium md:mb-2 md:text-xl", "Innovation" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Cutting-edge design patterns and modern web technologies. Stay ahead with the latest trends in UI/UX design and development practices." }
                        }
                    }
                    // Results
                    div {
                        class: "flex gap-6 space-y-4 rounded-lg md:block",
                        span {
                            class: "bg-accent flex size-10 shrink-0 items-center justify-center rounded-full md:size-12",
                            // Layers
                            svg { class: "size-4 md:size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" }
                                path { d: "m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65" }
                                path { d: "m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65" }
                            }
                        }
                        div {
                            h3 { class: "font-medium md:mb-2 md:text-xl", "Results" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Proven track record of successful implementations. These components have been battle-tested in real-world applications and deliver consistent results." }
                        }
                    }
                    // Efficiency
                    div {
                        class: "flex gap-6 space-y-4 rounded-lg md:block",
                        span {
                            class: "bg-accent flex size-10 shrink-0 items-center justify-center rounded-full md:size-12",
                            // BatteryCharging
                            svg { class: "size-4 md:size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2" }
                                path { d: "M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1" }
                                path { d: "M22 11v2" }
                                path { d: "M7.5 7 13 17" }
                                path { d: "M10.5 7H14" }
                                path { d: "M7 17h3.5" }
                            }
                        }
                        div {
                            h3 { class: "font-medium md:mb-2 md:text-xl", "Efficiency" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Optimized for performance and developer productivity. Lightweight, fast-loading components that help you build faster without compromising on quality." }
                        }
                    }
                }
                div {
                    class: "mt-16 flex justify-center",
                    a {
                        href: "https://shadcnblocks.com",
                        class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                        "More Features"
                    }
                }
            }
        }
    }
}
