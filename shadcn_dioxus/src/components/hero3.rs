use dioxus::prelude::*;
use crate::components::{Avatar, AvatarImage};

#[component]
pub fn Hero3() -> Element {
    rsx! {
        section {
            div {
                class: "container grid items-center gap-10 lg:grid-cols-2 lg:gap-20",
                div {
                    class: "mx-auto flex flex-col items-center text-center md:ml-auto lg:max-w-3xl lg:items-start lg:text-left",
                    h1 {
                        class: "my-6 text-pretty text-4xl font-bold lg:text-6xl xl:text-7xl",
                        "Blocks built with Shadcn & Tailwind"
                    }
                    p {
                        class: "text-muted-foreground mb-8 max-w-xl lg:text-xl",
                        "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."
                    }
                    div {
                        class: "mb-12 flex w-fit flex-col items-center gap-4 sm:flex-row",
                        span {
                            class: "inline-flex items-center -space-x-4",
                            Avatar {
                                class: "size-12 border",
                                AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-1.webp", alt: "User 1" }
                            }
                            Avatar {
                                class: "size-12 border",
                                AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-2.webp", alt: "User 2" }
                            }
                            Avatar {
                                class: "size-12 border",
                                AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-3.webp", alt: "User 3" }
                            }
                            Avatar {
                                class: "size-12 border",
                                AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-4.webp", alt: "User 4" }
                            }
                            Avatar {
                                class: "size-12 border",
                                AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-5.webp", alt: "User 5" }
                            }
                        }
                        div {
                            div {
                                class: "flex items-center gap-1",
                                svg {
                                    class: "size-5 fill-yellow-400 text-yellow-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                                svg {
                                    class: "size-5 fill-yellow-400 text-yellow-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                                svg {
                                    class: "size-5 fill-yellow-400 text-yellow-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                                svg {
                                    class: "size-5 fill-yellow-400 text-yellow-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                                svg {
                                    class: "size-5 fill-yellow-400 text-yellow-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                                span {
                                    class: "mr-1 font-semibold",
                                    "5.0"
                                }
                            }
                            p {
                                class: "text-muted-foreground text-left font-medium",
                                "from 200+ reviews"
                            }
                        }
                    }
                    div {
                        class: "flex w-full flex-col justify-center gap-2 sm:flex-row lg:justify-start",
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90 w-full sm:w-auto",
                            href: "https://www.shadcnblocks.com",
                            "Sign Up"
                        }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                            href: "https://www.shadcnblocks.com",
                            "Get Started"
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "m7 7 10 10" } path { d: "M17 7v10H7" } }
                        }
                    }
                }
                div {
                    class: "flex",
                    img {
                        class: "max-h-[600px] w-full rounded-md object-cover lg:max-h-[800px]",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        alt: "placeholder hero",
                    }
                }
            }
        }
    }
}
