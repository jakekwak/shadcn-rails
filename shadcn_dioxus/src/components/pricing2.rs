use dioxus::prelude::*;
use crate::components::Separator;

#[component]
pub fn Pricing2() -> Element {
    let mut is_yearly = use_signal(|| false);

    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto flex max-w-5xl flex-col items-center gap-6 text-center",
                    h2 {
                        class: "text-pretty text-4xl font-semibold lg:text-6xl",
                        "Pricing"
                    }
                    p {
                        class: "text-muted-foreground lg:text-xl",
                        "Check out our affordable pricing plans"
                    }
                    div {
                        class: "flex items-center gap-3 text-lg",
                        span {
                            class: if is_yearly() { "text-muted-foreground" } else { "font-semibold" },
                            "Monthly"
                        }
                        button {
                            class: "peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
                            class: if is_yearly() { "bg-primary justify-end" } else { "bg-input justify-start" },
                            onclick: move |_| is_yearly.set(!is_yearly()),
                            span {
                                class: "pointer-events-none block h-4 w-4 rounded-full bg-background shadow-lg ring-0 transition-transform",
                            }
                        }
                        span {
                            class: if is_yearly() { "font-semibold" } else { "text-muted-foreground" },
                            "Yearly"
                        }
                    }
                }
                div {
                    class: "mx-auto flex max-w-screen-md flex-col items-stretch gap-6 md:flex-row",
                    // Basic Plan
                    div {
                        class: "rounded-xl border bg-card text-card-foreground shadow flex flex-1 flex-col justify-between text-left",
                        div {
                            class: "flex flex-col space-y-1.5 p-6",
                            h3 {
                                class: "font-semibold leading-none tracking-tight text-lg",
                                "Basic"
                            }
                            p {
                                class: "text-muted-foreground text-sm",
                                "For individuals and small projects"
                            }
                            div {
                                class: "mt-4 flex items-end gap-1",
                                span {
                                    class: "text-4xl font-semibold",
                                    if is_yearly() { "$99" } else { "$9" }
                                }
                                span {
                                    class: "text-muted-foreground text-2xl font-semibold",
                                    if is_yearly() { "/yr" } else { "/mo" }
                                }
                            }
                        }
                        div {
                            class: "p-6 pt-0",
                            Separator { class: "mb-6" }
                            p {
                                class: "mb-3 font-semibold",
                                "What's included:"
                            }
                            ul {
                                class: "space-y-4",
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "1 user" }
                                }
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "5 projects" }
                                }
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "Basic support" }
                                }
                            }
                        }
                        div {
                            class: "flex items-center p-6 pt-0 mt-auto",
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground w-full",
                                href: "https://example.com",
                                "Get Started"
                            }
                        }
                    }
                    // Pro Plan
                    div {
                        class: "rounded-xl border-2 border-primary bg-card text-card-foreground shadow flex flex-1 flex-col justify-between text-left",
                        div {
                            class: "flex flex-col space-y-1.5 p-6",
                            div {
                                class: "flex items-center justify-between",
                                h3 {
                                    class: "font-semibold leading-none tracking-tight text-lg",
                                    "Pro"
                                }
                                span {
                                    class: "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold bg-primary text-primary-foreground",
                                    "Popular"
                                }
                            }
                            p {
                                class: "text-muted-foreground text-sm",
                                "For growing teams and businesses"
                            }
                            div {
                                class: "mt-4 flex items-end gap-1",
                                span {
                                    class: "text-4xl font-semibold",
                                    if is_yearly() { "$290" } else { "$29" }
                                }
                                span {
                                    class: "text-muted-foreground text-2xl font-semibold",
                                    if is_yearly() { "/yr" } else { "/mo" }
                                }
                            }
                        }
                        div {
                            class: "p-6 pt-0",
                            Separator { class: "mb-6" }
                            p {
                                class: "mb-3 font-semibold",
                                "Everything in Basic, and:"
                            }
                            ul {
                                class: "space-y-4",
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "5 users" }
                                }
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "Unlimited projects" }
                                }
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "Priority support" }
                                }
                                li {
                                    class: "flex items-center gap-2 text-sm",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } }
                                    span { "Advanced analytics" }
                                }
                            }
                        }
                        div {
                            class: "flex items-center p-6 pt-0 mt-auto",
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90 w-full",
                                href: "https://example.com",
                                "Get Started"
                            }
                        }
                    }
                }
            }
        }
    }
}
