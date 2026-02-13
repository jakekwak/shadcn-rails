use dioxus::prelude::*;
use crate::components::{Badge, Button, Separator};

#[component]
pub fn Pricing4() -> Element {
    let mut is_yearly = use_signal(|| false);

    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto flex max-w-7xl flex-col gap-6",
                    h2 {
                        class: "text-pretty text-4xl font-bold lg:text-6xl",
                        "Pricing"
                    }
                    div {
                        class: "flex flex-col justify-between gap-10 md:flex-row",
                        p {
                            class: "text-muted-foreground max-w-3xl lg:text-xl",
                            "Check out our affordable pricing plans."
                        }
                        div {
                            class: "bg-muted flex h-11 w-fit shrink-0 items-center rounded-md p-1 text-lg",
                            button {
                                class: "flex h-full cursor-pointer items-center justify-center rounded px-7 font-semibold transition-colors",
                                class: if !is_yearly() { "bg-background text-primary shadow-sm" } else { "text-muted-foreground" },
                                onclick: move |_| is_yearly.set(false),
                                "Monthly"
                            }
                            button {
                                class: "flex h-full cursor-pointer items-center justify-center rounded px-7 font-semibold transition-colors",
                                class: if is_yearly() { "bg-background text-primary shadow-sm" } else { "text-muted-foreground" },
                                onclick: move |_| is_yearly.set(true),
                                "Yearly"
                            }
                        }
                    }
                }
                div {
                    class: "flex w-full flex-col items-stretch gap-6 md:flex-row",
                    // Basic Plan
                    div {
                        class: "flex w-full flex-col rounded-lg border p-6 text-left",
                        Badge {
                            class: "mb-8 block w-fit uppercase",
                            "Basic"
                        }
                        span {
                            class: "text-4xl font-medium",
                            if is_yearly() { "$99" } else { "$9" }
                        }
                        p {
                            class: "text-muted-foreground",
                            if is_yearly() { "Per year" } else { "Per month" }
                        }
                        Separator { class: "my-6" }
                        div {
                            class: "flex h-full flex-col justify-between gap-20",
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "1 user" }
                                }
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "5 projects" }
                                }
                            }
                            Button {
                                class: "w-full",
                                "Get Started"
                            }
                        }
                    }
                    // Pro Plan
                    div {
                        class: "flex w-full flex-col rounded-lg border p-6 text-left",
                        Badge {
                            class: "mb-8 block w-fit uppercase",
                            "Pro"
                        }
                        span {
                            class: "text-4xl font-medium",
                            if is_yearly() { "$290" } else { "$29" }
                        }
                        p {
                            class: "text-muted-foreground",
                            if is_yearly() { "Per year" } else { "Per month" }
                        }
                        Separator { class: "my-6" }
                        div {
                            class: "flex h-full flex-col justify-between gap-20",
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "5 users" }
                                }
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "Unlimited projects" }
                                }
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "Priority support" }
                                }
                            }
                            Button {
                                class: "w-full",
                                "Get Started"
                            }
                        }
                    }
                    // Enterprise Plan
                    div {
                        class: "flex w-full flex-col rounded-lg border p-6 text-left",
                        Badge {
                            class: "mb-8 block w-fit uppercase",
                            "Enterprise"
                        }
                        span {
                            class: "text-4xl font-medium",
                            if is_yearly() { "$990" } else { "$99" }
                        }
                        p {
                            class: "text-muted-foreground",
                            if is_yearly() { "Per year" } else { "Per month" }
                        }
                        Separator { class: "my-6" }
                        div {
                            class: "flex h-full flex-col justify-between gap-20",
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "Unlimited users" }
                                }
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "Unlimited projects" }
                                }
                                li {
                                    class: "flex items-center gap-2",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    span { "Dedicated support" }
                                }
                            }
                            Button {
                                class: "w-full",
                                "Get Started"
                            }
                        }
                    }
                }
            }
        }
    }
}
