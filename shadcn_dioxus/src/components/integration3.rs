use dioxus::prelude::*;

#[component]
pub fn Integration3() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto flex flex-col items-center text-center",
                    div {
                        class: "flex max-w-5xl flex-col items-center text-center",
                        h1 {
                            class: "my-6 text-pretty text-4xl font-bold lg:text-6xl",
                            "Integrations"
                        }
                        h2 {
                            class: "text-muted-foreground mb-8 max-w-3xl lg:text-2xl",
                            "Connect your favourite apps to your workflow."
                        }
                    }
                    div {
                        class: "flex flex-col justify-center gap-4",
                        // Google Sheets
                        div {
                            class: "flex items-center gap-4 py-4",
                            div {
                                class: "h-12 w-12 flex-shrink-0",
                                img { class: "h-full w-full object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/google-icon.svg", alt: "Google Sheets", width: "48", height: "48" }
                            }
                            div {
                                class: "text-left",
                                div { class: "text-lg font-semibold", "Google Sheets" }
                                div { class: "text-muted-foreground text-sm", "Easily sync your data with Google Sheets for seamless automation." }
                            }
                        }
                        // Slack
                        div {
                            class: "flex items-center gap-4 py-4",
                            div {
                                class: "h-12 w-12 flex-shrink-0",
                                img { class: "h-full w-full object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/slack-icon.svg", alt: "Slack", width: "48", height: "48" }
                            }
                            div {
                                class: "text-left",
                                div { class: "text-lg font-semibold", "Slack" }
                                div { class: "text-muted-foreground text-sm", "Receive updates and notifications directly in your Slack channels." }
                            }
                        }
                        // Sketch
                        div {
                            class: "flex items-center gap-4 py-4",
                            div {
                                class: "h-12 w-12 flex-shrink-0",
                                img { class: "h-full w-full object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/sketch-icon.svg", alt: "Sketch", width: "48", height: "48" }
                            }
                            div {
                                class: "text-left",
                                div { class: "text-lg font-semibold", "Sketch" }
                                div { class: "text-muted-foreground text-sm", "Import your designs from Sketch and streamline your design process." }
                            }
                        }
                        // Gatsby
                        div {
                            class: "flex items-center gap-4 py-4",
                            div {
                                class: "h-12 w-12 flex-shrink-0",
                                img { class: "h-full w-full object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/gatsby-icon.svg", alt: "Gatsby", width: "48", height: "48" }
                            }
                            div {
                                class: "text-left",
                                div { class: "text-lg font-semibold", "Gatsby" }
                                div { class: "text-muted-foreground text-sm", "Build blazing-fast websites with Gatsby integration." }
                            }
                        }
                        // Shopify
                        div {
                            class: "flex items-center gap-4 py-4",
                            div {
                                class: "h-12 w-12 flex-shrink-0",
                                img { class: "h-full w-full object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/spotify-icon.svg", alt: "Shopify", width: "48", height: "48" }
                            }
                            div {
                                class: "text-left",
                                div { class: "text-lg font-semibold", "Shopify" }
                                div { class: "text-muted-foreground text-sm", "Sync your Shopify store data and streamline order management." }
                            }
                        }
                        // Github
                        div {
                            class: "flex items-center gap-4 py-4",
                            div {
                                class: "h-12 w-12 flex-shrink-0",
                                img { class: "h-full w-full object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/github-icon.svg", alt: "Github", width: "48", height: "48" }
                            }
                            div {
                                class: "text-left",
                                div { class: "text-lg font-semibold", "Github" }
                                div { class: "text-muted-foreground text-sm", "Automate your workflows and track changes with Github integration." }
                            }
                        }
                    }
                }
            }
        }
    }
}
