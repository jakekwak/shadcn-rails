use dioxus::prelude::*;
use crate::components::{Button, Input};

#[component]
pub fn Signup1() -> Element {
    rsx! {
        section {
            class: "bg-muted h-screen",
            div {
                class: "flex h-full items-center justify-center",
                // Logo
                div {
                    class: "flex flex-col items-center gap-6 lg:justify-start",
                    a {
                        href: "https://www.shadcnblocks.com",
                        img {class: "h-10", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/shadcnblockscom-wordmark.svg", alt: "logo", title: "shadcnblocks.com"}
                    }
                    div {
                        class: "min-w-sm border-muted bg-background flex w-full max-w-sm flex-col items-center gap-y-4 rounded-md border px-6 py-8 shadow-md",
                        h1 {class: "text-xl font-semibold",
                            "Signup"}
                        Input {class: "text-sm", r#type: "email", placeholder: "Email", }
                        Input {class: "text-sm", r#type: "password", placeholder: "Password", }
                        Input {class: "text-sm", r#type: "password", placeholder: "Confirm Password", }
                        Button {class: "w-full",
                            "Create Account"}
                    }
                    div {
                        class: "text-muted-foreground flex justify-center gap-1 text-sm",
                        p {"Already a user?"}
                        a {class: "text-primary font-medium hover:underline", href: "https://shadcnblocks.com",
                            "Login"}
                    }
                }
            }
        }
    }
}
