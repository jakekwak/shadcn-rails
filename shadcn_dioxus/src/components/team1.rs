use dioxus::prelude::*;
use crate::components::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn Team1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container flex flex-col items-center text-center",
                h2 {class: "my-6 text-pretty text-2xl font-bold lg:text-4xl",
                    "Team"}
                p {class: "text-muted-foreground mb-8 max-w-3xl lg:text-xl",
                    "Our diverse team of experts brings together decades of experience in design, engineering, and product development."}
            }
            div {
                class: "container mt-16 grid gap-x-8 gap-y-16 md:grid-cols-2 lg:grid-cols-3",
                div {
                    class: "flex flex-col items-center",
                    Avatar {
                        class: "mb-4 size-20 border md:mb-5 lg:size-24",
                        AvatarImage {src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-1.webp"}
                        AvatarFallback {"SC"}
                    }
                    p {class: "text-center font-medium", "Sarah Chen"}
                    p {class: "text-muted-foreground text-center", "CEO & Founder"}
                }
                div {
                    class: "flex flex-col items-center",
                    Avatar {
                        class: "mb-4 size-20 border md:mb-5 lg:size-24",
                        AvatarImage {src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-2.webp"}
                        AvatarFallback {"MR"}
                    }
                    p {class: "text-center font-medium", "Marcus Rodriguez"}
                    p {class: "text-muted-foreground text-center", "CTO"}
                }
                div {
                    class: "flex flex-col items-center",
                    Avatar {
                        class: "mb-4 size-20 border md:mb-5 lg:size-24",
                        AvatarImage {src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-3.webp"}
                        AvatarFallback {"EW"}
                    }
                    p {class: "text-center font-medium", "Emily Watson"}
                    p {class: "text-muted-foreground text-center", "Head of Design"}
                }
                div {
                    class: "flex flex-col items-center",
                    Avatar {
                        class: "mb-4 size-20 border md:mb-5 lg:size-24",
                        AvatarImage {src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-4.webp"}
                        AvatarFallback {"DK"}
                    }
                    p {class: "text-center font-medium", "David Kim"}
                    p {class: "text-muted-foreground text-center", "Lead Engineer"}
                }
                div {
                    class: "flex flex-col items-center",
                    Avatar {
                        class: "mb-4 size-20 border md:mb-5 lg:size-24",
                        AvatarImage {src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-5.webp"}
                        AvatarFallback {"LT"}
                    }
                    p {class: "text-center font-medium", "Lisa Thompson"}
                    p {class: "text-muted-foreground text-center", "Product Manager"}
                }
                div {
                    class: "flex flex-col items-center",
                    Avatar {
                        class: "mb-4 size-20 border md:mb-5 lg:size-24",
                        AvatarImage {src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-6.webp"}
                        AvatarFallback {"AJ"}
                    }
                    p {class: "text-center font-medium", "Alex Johnson"}
                    p {class: "text-muted-foreground text-center", "UX Designer"}
                }
            }
        }
    }
}
