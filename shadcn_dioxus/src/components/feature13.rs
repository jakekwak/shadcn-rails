use dioxus::prelude::*;

#[component]
pub fn Feature13() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto mb-16 max-w-3xl text-center",
                    h2 {
                        class: "text-pretty text-4xl font-medium lg:text-5xl",
                        "A collection of extra blocks for Shadcn UI & Tailwind"
                    }
                }
                div {
                    class: "grid gap-8 lg:grid-cols-2",
                    // Feature 1: Design System Approved
                    div {
                        class: "bg-muted flex flex-col justify-between rounded-lg",
                        div {
                            class: "flex justify-between gap-10 border-b",
                            div {
                                class: "flex flex-col justify-start justify-between gap-8 py-6 pl-4 md:gap-14 md:py-10 md:pl-8 lg:justify-normal",
                                span { class: "text-muted-foreground font-mono text-xs", "FOR DESIGNERS" }
                                a {
                                    href: "https://shadcnblocks.com",
                                    h3 { class: "hover:text-primary text-2xl transition-all hover:opacity-80 sm:text-3xl lg:text-4xl", "Design System Approved" }
                                }
                            }
                            div {
                                class: "md:1/3 w-2/5 shrink-0 rounded-r-lg border-l",
                                a {
                                    href: "https://shadcnblocks.com",
                                    img { class: "h-full w-full rounded-t-lg object-cover transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg", alt: "Design System Approved" }
                                }
                            }
                        }
                        p { class: "text-muted-foreground p-4 md:p-8", "Hundreds of finely crafted components for shadcn/ui available in Figma. Easily modify the design system to your brand." }
                    }
                    // Feature 2: Copy-Paste Code Blocks
                    div {
                        class: "bg-muted flex flex-col justify-between rounded-lg",
                        div {
                            class: "flex justify-between gap-10 border-b",
                            div {
                                class: "flex flex-col justify-start justify-between gap-8 py-6 pl-4 md:gap-14 md:py-10 md:pl-8 lg:justify-normal",
                                span { class: "text-muted-foreground font-mono text-xs", "FOR DEVELOPERS" }
                                a {
                                    href: "https://shadcnblocks.com",
                                    h3 { class: "hover:text-primary text-2xl transition-all hover:opacity-80 sm:text-3xl lg:text-4xl", "Copy-Paste Code Blocks" }
                                }
                            }
                            div {
                                class: "md:1/3 w-2/5 shrink-0 rounded-r-lg border-l",
                                a {
                                    href: "https://shadcnblocks.com",
                                    img { class: "h-full w-full rounded-t-lg object-cover transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-2.svg", alt: "Copy-Paste Code Blocks" }
                                }
                            }
                        }
                        p { class: "text-muted-foreground p-4 md:p-8", "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project." }
                    }
                    // Feature 3: Product-First Approach
                    div {
                        class: "bg-muted flex flex-col justify-between rounded-lg",
                        div {
                            class: "flex justify-between gap-10 border-b",
                            div {
                                class: "flex flex-col justify-start justify-between gap-8 py-6 pl-4 md:gap-14 md:py-10 md:pl-8 lg:justify-normal",
                                span { class: "text-muted-foreground font-mono text-xs", "FOR PRODUCT TEAMS" }
                                a {
                                    href: "https://shadcnblocks.com",
                                    h3 { class: "hover:text-primary text-2xl transition-all hover:opacity-80 sm:text-3xl lg:text-4xl", "Product-First Approach" }
                                }
                            }
                            div {
                                class: "md:1/3 w-2/5 shrink-0 rounded-r-lg border-l",
                                a {
                                    href: "https://shadcnblocks.com",
                                    img { class: "h-full w-full rounded-t-lg object-cover transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-3.svg", alt: "Product-First Approach" }
                                }
                            }
                        }
                        p { class: "text-muted-foreground p-4 md:p-8", "Components designed with user experience in mind. Every block is tested for usability and optimized for conversion rates." }
                    }
                    // Feature 4: Marketing-Ready Templates
                    div {
                        class: "bg-muted flex flex-col justify-between rounded-lg",
                        div {
                            class: "flex justify-between gap-10 border-b",
                            div {
                                class: "flex flex-col justify-start justify-between gap-8 py-6 pl-4 md:gap-14 md:py-10 md:pl-8 lg:justify-normal",
                                span { class: "text-muted-foreground font-mono text-xs", "FOR MARKETING" }
                                a {
                                    href: "https://shadcnblocks.com",
                                    h3 { class: "hover:text-primary text-2xl transition-all hover:opacity-80 sm:text-3xl lg:text-4xl", "Marketing-Ready Templates" }
                                }
                            }
                            div {
                                class: "md:1/3 w-2/5 shrink-0 rounded-r-lg border-l",
                                a {
                                    href: "https://shadcnblocks.com",
                                    img { class: "h-full w-full rounded-t-lg object-cover transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-4.svg", alt: "Marketing-Ready Templates" }
                                }
                            }
                        }
                        p { class: "text-muted-foreground p-4 md:p-8", "High-converting landing pages, email templates, and marketing components that drive engagement and boost your campaigns." }
                    }
                }
            }
        }
    }
}
