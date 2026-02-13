use dioxus::prelude::*;
use crate::components::{Badge, Separator};

#[component]
pub fn Hero45() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container overflow-hidden",
                div {
                    class: "mb-20 flex flex-col items-center gap-6 text-center",
                    Badge { variant: "outline", "shadcnblocks.com" }
                    h1 {
                        class: "text-4xl font-semibold lg:text-5xl",
                        "Blocks built with Shadcn & Tailwind"
                    }
                }
                div {
                    class: "relative mx-auto max-w-5xl",
                    div { class: "absolute -right-28 -top-28 -z-10 aspect-video h-72 w-96 opacity-40 [background-size:12px_12px] [mask-image:radial-gradient(ellipse_100%_100%_at_50%_50%,#000_40%,transparent_100%)] sm:bg-[radial-gradient(hsl(var(--muted-foreground))_1px,transparent_1px)]" }
                    div { class: "absolute -left-28 -top-28 -z-10 aspect-video h-72 w-96 opacity-40 [background-size:12px_12px] [mask-image:radial-gradient(ellipse_100%_100%_at_50%_50%,#000_40%,transparent_100%)] sm:bg-[radial-gradient(hsl(var(--muted-foreground))_1px,transparent_1px)]" }
                    img {
                        class: "aspect-video max-h-[500px] w-full rounded-xl object-cover",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        alt: "placeholder",
                    }
                    div { class: "bg-linear-to-t from-background absolute inset-0 via-transparent to-transparent" }
                }
                div {
                    class: "mx-auto mt-10 flex max-w-5xl flex-col md:flex-row",
                    div {
                        class: "bg-background flex grow basis-0 flex-col rounded-md p-4",
                        div { class: "bg-background mb-6 flex size-10 items-center justify-center rounded-full drop-shadow-lg",
                            svg { class: "h-auto w-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M11 12h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 14" }
                                path { d: "m7 18 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9" }
                                path { d: "m2 13 6 6" }
                            }
                        }
                        h3 { class: "mb-2 font-semibold", "Flexible Support" }
                        p { class: "text-muted-foreground text-sm", "Benefit from around-the-clock assistance to keep your business running smoothly." }
                    }
                    Separator {
                        class: "bg-linear-to-b from-muted to-muted mx-6 hidden h-auto w-[2px] via-transparent md:block",
                        orientation: "vertical",
                    }
                    div {
                        class: "bg-background flex grow basis-0 flex-col rounded-md p-4",
                        div { class: "bg-background mb-6 flex size-10 items-center justify-center rounded-full drop-shadow-lg",
                            svg { class: "h-auto w-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                                circle { cx: "9", cy: "7", r: "4" }
                                path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
                                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                            }
                        }
                        h3 { class: "mb-2 font-semibold", "Collaborative Tools" }
                        p { class: "text-muted-foreground text-sm", "Enhance teamwork with tools designed to simplify project management and communication." }
                    }
                    Separator {
                        class: "bg-linear-to-b from-muted to-muted mx-6 hidden h-auto w-[2px] via-transparent md:block",
                        orientation: "vertical",
                    }
                    div {
                        class: "bg-background flex grow basis-0 flex-col rounded-md p-4",
                        div { class: "bg-background mb-6 flex size-10 items-center justify-center rounded-full drop-shadow-lg",
                            svg { class: "h-auto w-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" }
                            }
                        }
                        h3 { class: "mb-2 font-semibold", "Lightning Fast Speed" }
                        p { class: "text-muted-foreground text-sm", "Experience the fastest load times with our high performance servers." }
                    }
                }
            }
        }
    }
}
