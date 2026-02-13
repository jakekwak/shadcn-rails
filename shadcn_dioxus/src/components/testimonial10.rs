use dioxus::prelude::*;
use crate::components::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn Testimonial10() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "flex flex-col items-center text-center",
                    p {
                        class: "mb-16 max-w-4xl px-8 font-medium lg:text-3xl",
                        "\u{201C}Lorem ipsum dolor sit amet consectetur adipisicing elit. Elig doloremque mollitia fugiat omnis! Porro facilis quo animi consequatur. Explicabo.\u{201D}"
                    }
                    div {
                        class: "flex items-center gap-2 md:gap-4",
                        Avatar {
                            class: "size-12 md:size-16",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-1.webp", alt: "Customer" }
                            AvatarFallback { "CN" }
                        }
                        div {
                            class: "text-left",
                            p { class: "text-sm font-medium md:text-base", "Customer Name" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Role" }
                        }
                    }
                }
            }
        }
    }
}
