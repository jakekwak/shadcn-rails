use dioxus::prelude::*;
use crate::components::Separator;

#[component]
pub fn List2() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container px-0 md:px-8",
                h1 {
                    class: "mb-10 px-4 text-3xl font-semibold md:mb-14 md:text-4xl",
                    "Our Achievements & Recognition"
                }
                div {
                    class: "flex flex-col",
                    Separator {}
                    // 1. Industry Recognition - Trophy
                    div {
                        class: "grid items-center gap-4 px-4 py-5 md:grid-cols-4",
                        div {
                            class: "order-2 flex items-center gap-2 md:order-none",
                            span { class: "bg-muted flex h-14 w-16 shrink-0 items-center justify-center rounded-md",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M6 9H4.5a2.5 2.5 0 0 1 0-5H6" } path { d: "M18 9h1.5a2.5 2.5 0 0 0 0-5H18" } path { d: "M4 22h16" } path { d: "M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22" } path { d: "M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22" } path { d: "M18 2H6v7a6 6 0 0 0 12 0V2Z" } }
                            }
                            div {
                                class: "flex flex-col gap-1",
                                h3 { class: "font-semibold", "Industry Recognition" }
                                p { class: "text-muted-foreground text-sm", "Achievement" }
                            }
                        }
                        p { class: "order-1 text-2xl font-semibold md:order-none md:col-span-2", "Outstanding Performance Award." }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground order-3 ml-auto w-fit gap-2 md:order-none",
                            href: "https://example.com",
                            span { "View project" }
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                        }
                    }
                    Separator {}
                    // 2. Excellence Award - Award
                    div {
                        class: "grid items-center gap-4 px-4 py-5 md:grid-cols-4",
                        div {
                            class: "order-2 flex items-center gap-2 md:order-none",
                            span { class: "bg-muted flex h-14 w-16 shrink-0 items-center justify-center rounded-md",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "m15.477 12.89 1.515 8.526a.5.5 0 0 1-.81.47l-3.58-2.687a1 1 0 0 0-1.197 0l-3.586 2.686a.5.5 0 0 1-.81-.469l1.514-8.526" } circle { cx: "12", cy: "8", r: "6" } }
                            }
                            div {
                                class: "flex flex-col gap-1",
                                h3 { class: "font-semibold", "Excellence Award" }
                                p { class: "text-muted-foreground text-sm", "Recognition" }
                            }
                        }
                        p { class: "order-1 text-2xl font-semibold md:order-none md:col-span-2", "Best in Category Winner." }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground order-3 ml-auto w-fit gap-2 md:order-none",
                            href: "https://example.com",
                            span { "View project" }
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                        }
                    }
                    Separator {}
                    // 3. Innovation Prize - Lightbulb
                    div {
                        class: "grid items-center gap-4 px-4 py-5 md:grid-cols-4",
                        div {
                            class: "order-2 flex items-center gap-2 md:order-none",
                            span { class: "bg-muted flex h-14 w-16 shrink-0 items-center justify-center rounded-md",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" } path { d: "M9 18h6" } path { d: "M10 22h4" } }
                            }
                            div {
                                class: "flex flex-col gap-1",
                                h3 { class: "font-semibold", "Innovation Prize" }
                                p { class: "text-muted-foreground text-sm", "Technology" }
                            }
                        }
                        p { class: "order-1 text-2xl font-semibold md:order-none md:col-span-2", "Breakthrough Solution of the Year." }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground order-3 ml-auto w-fit gap-2 md:order-none",
                            href: "https://example.com",
                            span { "View project" }
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                        }
                    }
                    Separator {}
                    // 4. Customer Success - HeartHandshake
                    div {
                        class: "grid items-center gap-4 px-4 py-5 md:grid-cols-4",
                        div {
                            class: "order-2 flex items-center gap-2 md:order-none",
                            span { class: "bg-muted flex h-14 w-16 shrink-0 items-center justify-center rounded-md",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" } path { d: "M12 5 9.04 7.96a2.17 2.17 0 0 0 0 3.08c.82.82 2.13.85 3 .07l2.07-1.9a2.82 2.82 0 0 1 3.79 0 2.6 2.6 0 0 1 0 3.56L12 18" } }
                            }
                            div {
                                class: "flex flex-col gap-1",
                                h3 { class: "font-semibold", "Customer Success" }
                                p { class: "text-muted-foreground text-sm", "Service" }
                            }
                        }
                        p { class: "order-1 text-2xl font-semibold md:order-none md:col-span-2", "Top-Rated Solution Provider." }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground order-3 ml-auto w-fit gap-2 md:order-none",
                            href: "https://example.com",
                            span { "View project" }
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                        }
                    }
                    Separator {}
                    // 5. Global Leadership - Building2
                    div {
                        class: "grid items-center gap-4 px-4 py-5 md:grid-cols-4",
                        div {
                            class: "order-2 flex items-center gap-2 md:order-none",
                            span { class: "bg-muted flex h-14 w-16 shrink-0 items-center justify-center rounded-md",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z" } path { d: "M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2" } path { d: "M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2" } path { d: "M10 6h4" } path { d: "M10 10h4" } path { d: "M10 14h4" } path { d: "M10 18h4" } }
                            }
                            div {
                                class: "flex flex-col gap-1",
                                h3 { class: "font-semibold", "Global Leadership" }
                                p { class: "text-muted-foreground text-sm", "Management" }
                            }
                        }
                        p { class: "order-1 text-2xl font-semibold md:order-none md:col-span-2", "Executive Team of the Year." }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground order-3 ml-auto w-fit gap-2 md:order-none",
                            href: "https://example.com",
                            span { "View project" }
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                        }
                    }
                    Separator {}
                    // 6. Sustainability Impact - Leaf
                    div {
                        class: "grid items-center gap-4 px-4 py-5 md:grid-cols-4",
                        div {
                            class: "order-2 flex items-center gap-2 md:order-none",
                            span { class: "bg-muted flex h-14 w-16 shrink-0 items-center justify-center rounded-md",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M11 20A7 7 0 0 1 9.8 6.9C15.5 4.9 17 3.5 19 2c1 2 2 4.5 2 8 0 5.5-4.78 10-10 10Z" } path { d: "M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12" } }
                            }
                            div {
                                class: "flex flex-col gap-1",
                                h3 { class: "font-semibold", "Sustainability Impact" }
                                p { class: "text-muted-foreground text-sm", "Environmental" }
                            }
                        }
                        p { class: "order-1 text-2xl font-semibold md:order-none md:col-span-2", "Green Initiative Excellence." }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground order-3 ml-auto w-fit gap-2 md:order-none",
                            href: "https://example.com",
                            span { "View project" }
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                        }
                    }
                    Separator {}
                }
            }
        }
    }
}
