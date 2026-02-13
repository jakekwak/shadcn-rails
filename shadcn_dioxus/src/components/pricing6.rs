use dioxus::prelude::*;
use crate::components::{Button, Separator};

#[component]
pub fn Pricing6() -> Element {
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
                        class: "text-muted-foreground max-w-md lg:text-xl",
                        "Simple pricing with a free 7 day trial."
                    }
                    div {
                        class: "mx-auto flex w-full flex-col rounded-lg border p-6 sm:w-fit sm:min-w-80",
                        div {
                            class: "flex justify-center",
                            span { class: "text-lg font-semibold", "$" }
                            span { class: "text-6xl font-semibold", "29" }
                            span { class: "text-muted-foreground self-end", "/mo" }
                        }
                        div {
                            class: "my-6",
                            // Group 1
                            div {
                                ul {
                                    class: "flex flex-col gap-3",
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Unlimited"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Integrations"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "24/7 support"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                }
                                Separator { class: "my-6" }
                            }
                            // Group 2
                            div {
                                ul {
                                    class: "flex flex-col gap-3",
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Live collaborations"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Unlimited storage"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "30-day money back"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                }
                                Separator { class: "my-6" }
                            }
                            // Group 3
                            div {
                                ul {
                                    class: "flex flex-col gap-3",
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Unlimited members"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Customization"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                    li {
                                        class: "flex items-center justify-between gap-2 text-sm font-medium",
                                        "Unlimited users"
                                        svg { class: "inline size-4 shrink-0", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M20 6 9 17l-5-5" } }
                                    }
                                }
                            }
                        }
                        Button { "Start free trial" }
                    }
                }
            }
        }
    }
}
