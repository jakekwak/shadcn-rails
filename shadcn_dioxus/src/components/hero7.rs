use dioxus::prelude::*;
use crate::components::{Avatar, AvatarImage};

#[component]
pub fn Hero7() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container text-center",
                div {
                    class: "mx-auto flex max-w-5xl flex-col gap-6",
                    h1 {
                        class: "text-3xl font-semibold lg:text-6xl",
                        "A Collection of Components Built With Shadcn & Tailwind"
                    }
                    p {
                        class: "text-muted-foreground text-balance lg:text-lg",
                        "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."
                    }
                }
                a {
                    class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 rounded-md px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90 mt-10",
                    href: "https://www.shadcnblocks.com",
                    "Discover all components"
                }
                div {
                    class: "mx-auto mt-10 flex w-fit flex-col items-center gap-4 sm:flex-row",
                    span {
                        class: "mx-4 inline-flex items-center -space-x-4",
                        Avatar {
                            class: "size-14 border",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-1.webp", alt: "User 1" }
                        }
                        Avatar {
                            class: "size-14 border",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-2.webp", alt: "User 2" }
                        }
                        Avatar {
                            class: "size-14 border",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-3.webp", alt: "User 3" }
                        }
                        Avatar {
                            class: "size-14 border",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-4.webp", alt: "User 4" }
                        }
                        Avatar {
                            class: "size-14 border",
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
            }
        }
    }
}
