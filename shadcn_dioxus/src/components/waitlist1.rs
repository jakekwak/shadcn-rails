use dioxus::prelude::*;
use crate::components::{Avatar, AvatarImage, Button, Input};

#[component]
pub fn Waitlist1() -> Element {
    rsx! {
        section {
            class: "flex h-full min-h-screen w-screen items-center justify-center overflow-hidden py-32",
            div {
                class: "container flex w-full flex-col items-center justify-center px-4 md:h-full",
                h2 {
                    class: "relative z-20 py-2 text-center font-sans text-5xl font-semibold tracking-tighter md:py-10 lg:text-8xl",
                    "Join the Waitlist"
                }
                p {
                    class: "text-md text-muted-foreground mx-auto max-w-xl text-center lg:text-lg",
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                }
                div {
                    class: "relative z-20 mt-10 flex w-full max-w-md items-center gap-3 rounded-full p-1",
                    Input {
                        class: "bg-muted h-10 w-full rounded-xl border-none shadow-none ring-0 focus-visible:outline-none focus-visible:ring-0 active:outline-0 active:ring-0",
                        placeholder: "Enter your email",
                    }
                    Button {
                        class: "h-10 rounded-xl",
                        "Join the Waitlist"
                    }
                }
                div {
                    class: "mt-10 flex items-center gap-2",
                    span {
                        class: "inline-flex items-center -space-x-2.5",
                        Avatar {
                            class: "size-8",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/guri3/avatar1.png", alt: "placeholder" }
                        }
                        Avatar {
                            class: "size-8",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/guri3/avatar2.png", alt: "placeholder" }
                        }
                        Avatar {
                            class: "size-8",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/guri3/avatar3.png", alt: "placeholder" }
                        }
                        Avatar {
                            class: "size-8",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/guri3/avatar4.png", alt: "placeholder" }
                        }
                        Avatar {
                            class: "size-8",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/guri3/avatar5.png", alt: "placeholder" }
                        }
                        Avatar {
                            class: "size-8",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/guri3/avatar6.png", alt: "placeholder" }
                        }
                    }
                    p {
                        class: "text-muted-foreground/80 tracking-tight",
                        "+1000 people already joined"
                    }
                }
            }
        }
    }
}
