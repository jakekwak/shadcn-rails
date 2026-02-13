use dioxus::prelude::*;
use crate::components::Badge;

#[component]
pub fn Hero1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "grid items-center gap-8 lg:grid-cols-2",
                    div {
                        class: "flex flex-col items-center text-center lg:items-start lg:text-left",
                        Badge {
                            variant: "outline",
                            "\u{2728} Your Website Builder"
                            // ArrowUpRight
                            svg { class: "ml-2 size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M7 7h10v10" }
                                path { d: "M7 17 17 7" }
                            }
                        }
                        h1 {
                            class: "my-6 text-pretty text-4xl font-bold lg:text-6xl",
                            "Blocks Built With Shadcn & Tailwind"
                        }
                        p {
                            class: "text-muted-foreground mb-8 max-w-xl lg:text-xl",
                            "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."
                        }
                        div {
                            class: "flex w-full flex-col justify-center gap-2 sm:flex-row lg:justify-start",
                            a {
                                href: "https://www.shadcnblocks.com",
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90 w-full sm:w-auto",
                                "Discover all components"
                            }
                            a {
                                href: "https://www.shadcnblocks.com",
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground w-full sm:w-auto",
                                "View on GitHub"
                                // ArrowRight
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                    path { d: "M5 12h14" }
                                    path { d: "m12 5 7 7-7 7" }
                                }
                            }
                        }
                    }
                    img {
                        class: "max-h-96 w-full rounded-md object-cover",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        alt: "Hero section demo image showing interface components",
                    }
                }
            }
        }
    }
}
