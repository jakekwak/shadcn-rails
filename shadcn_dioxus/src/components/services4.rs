use dioxus::prelude::*;

#[component]
pub fn Services4() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto max-w-6xl space-y-12",
                    div {
                        class: "space-y-4 text-center",
                        h2 { class: "text-3xl font-semibold tracking-tight md:text-4xl", "Services" }
                        p { class: "text-muted-foreground mx-auto max-w-2xl text-lg tracking-tight md:text-xl", "We craft digital experiences that captivate and convert, bringing your vision to life." }
                    }
                    div {
                        class: "grid grid-cols-1 gap-8 md:grid-cols-2",
                        // Product Strategy
                        div {
                            class: "border-border space-y-6 rounded-lg border p-8 transition-shadow hover:shadow-sm",
                            div {
                                class: "flex items-center gap-4",
                                div { class: "bg-muted rounded-full p-3",
                                    svg { class: "h-6 w-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" } path { d: "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" } path { d: "M12 2v2" } path { d: "M12 22v-2" } path { d: "m17 20.66-1-1.73" } path { d: "M11 10.27 7 3.34" } path { d: "m20.66 17-1.73-1" } path { d: "m3.34 7 1.73 1" } path { d: "M14 12h8" } path { d: "M2 12h2" } path { d: "m20.66 7-1.73 1" } path { d: "m3.34 17 1.73-1" } path { d: "m17 3.34-1 1.73" } path { d: "m11 13.73-4 6.93" } }
                                }
                                h3 { class: "text-xl font-semibold", "Product Strategy" }
                            }
                            p { class: "text-muted-foreground leading-relaxed", "Strategic planning and market positioning to ensure your product meets user needs and business goals." }
                            div {
                                class: "space-y-2",
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Market Research" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "User Personas" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Competitive Analysis" } }
                            }
                        }
                        // Design
                        div {
                            class: "border-border space-y-6 rounded-lg border p-8 transition-shadow hover:shadow-sm",
                            div {
                                class: "flex items-center gap-4",
                                div { class: "bg-muted rounded-full p-3",
                                    svg { class: "h-6 w-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M15.707 21.293a1 1 0 0 1-1.414 0l-1.586-1.586a1 1 0 0 1 0-1.414l5.586-5.586a1 1 0 0 1 1.414 0l1.586 1.586a1 1 0 0 1 0 1.414z" } path { d: "m18 13-1.375-6.874a1 1 0 0 0-.746-.776L3.235 2.028a1 1 0 0 0-1.207 1.207L5.35 15.879a1 1 0 0 0 .776.746L13 18" } path { d: "m2.3 2.3 7.286 7.286" } circle { cx: "11", cy: "11", r: "2" } }
                                }
                                h3 { class: "text-xl font-semibold", "Design" }
                            }
                            p { class: "text-muted-foreground leading-relaxed", "Beautiful, user-centered designs that create engaging experiences across all platforms." }
                            div {
                                class: "space-y-2",
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "UI/UX Design" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Prototyping" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Interaction Design" } }
                            }
                        }
                        // Web Development
                        div {
                            class: "border-border space-y-6 rounded-lg border p-8 transition-shadow hover:shadow-sm",
                            div {
                                class: "flex items-center gap-4",
                                div { class: "bg-muted rounded-full p-3",
                                    svg { class: "h-6 w-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "16 18 22 12 16 6" } polyline { points: "8 6 2 12 8 18" } }
                                }
                                h3 { class: "text-xl font-semibold", "Web Development" }
                            }
                            p { class: "text-muted-foreground leading-relaxed", "Modern, scalable web applications built with the latest technologies and best practices." }
                            div {
                                class: "space-y-2",
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Frontend Dev" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Backend Dev" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "API Integration" } }
                            }
                        }
                        // Marketing
                        div {
                            class: "border-border space-y-6 rounded-lg border p-8 transition-shadow hover:shadow-sm",
                            div {
                                class: "flex items-center gap-4",
                                div { class: "bg-muted rounded-full p-3",
                                    svg { class: "h-6 w-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M12 22v-7l-2-2" } path { d: "M17 8v.8A6 6 0 0 1 13.8 20H10A6.5 6.5 0 0 1 7 8a5 5 0 0 1 10 0Z" } path { d: "m14 14-2 2" } }
                                }
                                h3 { class: "text-xl font-semibold", "Marketing" }
                            }
                            p { class: "text-muted-foreground leading-relaxed", "Data-driven strategies to launch successfully and scale your product efficiently." }
                            div {
                                class: "space-y-2",
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "SEO Strategy" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "Analytics & Data" } }
                                div { class: "flex items-center gap-2", div { class: "bg-foreground h-1.5 w-1.5 rounded-full" } span { class: "text-sm font-medium", "A/B Testing" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
