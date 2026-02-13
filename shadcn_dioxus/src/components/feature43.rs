use dioxus::prelude::*;

#[component]
pub fn Feature43() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto mb-16 max-w-3xl text-center",
                    h2 {
                        class: "text-pretty text-4xl font-medium lg:text-5xl",
                        "Fully featured components for Shadcn UI & Tailwind"
                    }
                }
                div {
                    class: "grid gap-10 md:grid-cols-2 lg:grid-cols-3",
                    div {
                        class: "flex flex-col",
                        div { class: "bg-accent mb-5 flex size-16 items-center justify-center rounded-full",
                            // GitPullRequest icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                circle { cx: "18", cy: "18", r: "3" }
                                circle { cx: "6", cy: "6", r: "3" }
                                path { d: "M13 6h3a2 2 0 0 1 2 2v7" }
                                path { d: "M6 9v12" }
                            }
                        }
                        h3 { class: "mb-2 text-xl font-semibold", "Quality" }
                        p { class: "text-muted-foreground", "Built with attention to detail and best practices. Every component is thoroughly tested and follows modern React patterns for reliability and performance." }
                    }
                    div {
                        class: "flex flex-col",
                        div { class: "bg-accent mb-5 flex size-16 items-center justify-center rounded-full",
                            // SquareKanban icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
                                path { d: "M8 7v7" }
                                path { d: "M12 7v4" }
                                path { d: "M16 7v9" }
                            }
                        }
                        h3 { class: "mb-2 text-xl font-semibold", "Experience" }
                        p { class: "text-muted-foreground", "Crafted with user experience in mind. Each component is designed to be intuitive, accessible, and provide smooth interactions across all devices." }
                    }
                    div {
                        class: "flex flex-col",
                        div { class: "bg-accent mb-5 flex size-16 items-center justify-center rounded-full",
                            // RadioTower icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M4.9 16.1C1 12.2 1 5.8 4.9 1.9" }
                                path { d: "M7.8 4.7a6.14 6.14 0 0 0-.8 7.5" }
                                path { d: "M16.2 4.7a6.14 6.14 0 0 1 .8 7.5" }
                                path { d: "M19.1 1.9a10.14 10.14 0 0 1 0 14.2" }
                                path { d: "M9.56 14.24a2.74 2.74 0 0 0 4.88 0" }
                                path { d: "M12 22v-8.5" }
                                circle { cx: "12", cy: "12", r: "2" }
                            }
                        }
                        h3 { class: "mb-2 text-xl font-semibold", "Support" }
                        p { class: "text-muted-foreground", "Comprehensive documentation and community support. Get help when you need it with detailed guides, examples, and active community assistance." }
                    }
                    div {
                        class: "flex flex-col",
                        div { class: "bg-accent mb-5 flex size-16 items-center justify-center rounded-full",
                            // WandSparkles icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72Z" }
                                path { d: "m14 7 3 3" }
                                path { d: "M5 6v4" }
                                path { d: "M19 14v4" }
                                path { d: "M10 2v2" }
                                path { d: "M7 8H3" }
                                path { d: "M21 16h-4" }
                                path { d: "M11 3H9" }
                            }
                        }
                        h3 { class: "mb-2 text-xl font-semibold", "Innovation" }
                        p { class: "text-muted-foreground", "Cutting-edge design patterns and modern web technologies. Stay ahead with the latest trends in UI/UX design and development practices." }
                    }
                    div {
                        class: "flex flex-col",
                        div { class: "bg-accent mb-5 flex size-16 items-center justify-center rounded-full",
                            // Layers icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" }
                                path { d: "m22.54 12.43-1.96-.89-8.58 3.91a2 2 0 0 1-1.66 0l-8.58-3.91-1.96.89a1 1 0 0 0 0 1.83l8.58 3.9a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" }
                            }
                        }
                        h3 { class: "mb-2 text-xl font-semibold", "Results" }
                        p { class: "text-muted-foreground", "Proven track record of successful implementations. These components have been battle-tested in real-world applications and deliver consistent results." }
                    }
                    div {
                        class: "flex flex-col",
                        div { class: "bg-accent mb-5 flex size-16 items-center justify-center rounded-full",
                            // BatteryCharging icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2" }
                                path { d: "M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1" }
                                path { d: "M22 11v2" }
                                path { d: "M7.5 7l3 5h-3l3 5" }
                            }
                        }
                        h3 { class: "mb-2 text-xl font-semibold", "Efficiency" }
                        p { class: "text-muted-foreground", "Optimized for performance and developer productivity. Lightweight, fast-loading components that help you build faster without compromising on quality." }
                    }
                }
                div {
                    class: "mt-16 flex justify-center",
                    a {
                        class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                        href: "https://shadcnblocks.com",
                        "More Features"
                    }
                }
            }
        }
    }
}
