use dioxus::prelude::*;
use crate::components::Badge;

#[component]
pub fn Changelog1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto max-w-3xl",
                    h1 {
                        class: "mb-4 text-3xl font-bold tracking-tight md:text-5xl",
                        "Changelog"
                    }
                    p {
                        class: "text-muted-foreground mb-6 text-base md:text-lg",
                        "Get the latest updates and improvements to our platform."
                    }
                }
                div {
                    class: "mx-auto mt-16 max-w-3xl space-y-16 md:mt-24 md:space-y-24",

                    // Entry 1: Version 1.3.0
                    div {
                        class: "relative flex flex-col gap-4 md:flex-row md:gap-16",
                        div {
                            class: "top-8 flex h-min w-64 shrink-0 items-center gap-4 md:sticky",
                            Badge { class: "text-xs", variant: "secondary", "Version 1.3.0" }
                            span { class: "text-muted-foreground text-xs font-medium", "15 November 2024" }
                        }
                        div {
                            class: "flex flex-col",
                            h2 { class: "text-foreground/90 mb-3 text-lg font-bold leading-tight md:text-2xl", "Enhanced Analytics Dashboard" }
                            p { class: "text-muted-foreground text-sm md:text-base", "We\u{2019}ve completely redesigned our analytics dashboard to provide deeper insights and improved visualizations of your data." }
                            ul {
                                class: "text-muted-foreground ml-4 mt-4 space-y-1.5 text-sm md:text-base",
                                li { class: "list-disc", "Interactive data visualizations with real-time updates" }
                                li { class: "list-disc", "Customizable dashboard widgets" }
                                li { class: "list-disc", "Export analytics in multiple formats (CSV, PDF, Excel)" }
                                li { class: "list-disc", "New reporting templates for common use cases" }
                                li { class: "list-disc", "Improved data filtering and segmentation options" }
                            }
                            img {
                                class: "mt-8 w-full rounded-lg object-cover",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-aspect-video-1.svg",
                                alt: "Version 1.3.0 visual",
                            }
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 text-primary underline-offset-4 hover:underline mt-4 self-end",
                                target: "_blank",
                                href: "https://shadcnblocks.com",
                                "Learn more"
                                svg { class: "h-4 w-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M7 7h10v10" } path { d: "M7 17 17 7" } }
                            }
                        }
                    }

                    // Entry 2: Version 1.2.5
                    div {
                        class: "relative flex flex-col gap-4 md:flex-row md:gap-16",
                        div {
                            class: "top-8 flex h-min w-64 shrink-0 items-center gap-4 md:sticky",
                            Badge { class: "text-xs", variant: "secondary", "Version 1.2.5" }
                            span { class: "text-muted-foreground text-xs font-medium", "7 October 2024" }
                        }
                        div {
                            class: "flex flex-col",
                            h2 { class: "text-foreground/90 mb-3 text-lg font-bold leading-tight md:text-2xl", "Mobile App Launch" }
                            p { class: "text-muted-foreground text-sm md:text-base", "We\u{2019}re excited to announce the launch of our mobile application, available now on iOS and Android platforms." }
                            ul {
                                class: "text-muted-foreground ml-4 mt-4 space-y-1.5 text-sm md:text-base",
                                li { class: "list-disc", "Native mobile experience for on-the-go productivity" }
                                li { class: "list-disc", "Offline mode support for working without internet connection" }
                                li { class: "list-disc", "Push notifications for important updates" }
                                li { class: "list-disc", "Biometric authentication for enhanced security" }
                            }
                        }
                    }

                    // Entry 3: Version 1.2.1
                    div {
                        class: "relative flex flex-col gap-4 md:flex-row md:gap-16",
                        div {
                            class: "top-8 flex h-min w-64 shrink-0 items-center gap-4 md:sticky",
                            Badge { class: "text-xs", variant: "secondary", "Version 1.2.1" }
                            span { class: "text-muted-foreground text-xs font-medium", "23 September 2024" }
                        }
                        div {
                            class: "flex flex-col",
                            h2 { class: "text-foreground/90 mb-3 text-lg font-bold leading-tight md:text-2xl", "New features and improvements" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Here are the latest updates and improvements to our platform. We are always working to improve our platform and your experience." }
                            ul {
                                class: "text-muted-foreground ml-4 mt-4 space-y-1.5 text-sm md:text-base",
                                li { class: "list-disc", "Added new feature to export data" }
                                li { class: "list-disc", "Improved performance and speed" }
                                li { class: "list-disc", "Fixed minor bugs and issues" }
                                li { class: "list-disc", "Added new feature to import data" }
                            }
                            img {
                                class: "mt-8 w-full rounded-lg object-cover",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-aspect-video-1.svg",
                                alt: "Version 1.2.1 visual",
                            }
                        }
                    }

                    // Entry 4: Version 1.0.0
                    div {
                        class: "relative flex flex-col gap-4 md:flex-row md:gap-16",
                        div {
                            class: "top-8 flex h-min w-64 shrink-0 items-center gap-4 md:sticky",
                            Badge { class: "text-xs", variant: "secondary", "Version 1.0.0" }
                            span { class: "text-muted-foreground text-xs font-medium", "31 August 2024" }
                        }
                        div {
                            class: "flex flex-col",
                            h2 { class: "text-foreground/90 mb-3 text-lg font-bold leading-tight md:text-2xl", "First version of our platform" }
                            p { class: "text-muted-foreground text-sm md:text-base", "Introducing a new platform to help you manage your projects and tasks. We are excited to launch our platform and help you get started. We are always working to improve our platform and your experience." }
                            img {
                                class: "mt-8 w-full rounded-lg object-cover",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-aspect-video-1.svg",
                                alt: "Version 1.0.0 visual",
                            }
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 text-primary underline-offset-4 hover:underline mt-4 self-end",
                                target: "_blank",
                                href: "https://shadcnblocks.com",
                                "Learn more"
                                svg { class: "h-4 w-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M7 7h10v10" } path { d: "M7 17 17 7" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
