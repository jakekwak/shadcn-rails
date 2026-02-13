use dioxus::prelude::*;
use crate::components::Badge;

#[component]
pub fn Blog7() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container mx-auto flex flex-col items-center gap-16 lg:px-16",
                div {
                    class: "text-center",
                    Badge {
                        class: "mb-6",
                        variant: "secondary",
                        "Latest Updates"
                    }
                    h2 {
                        class: "mb-3 text-pretty text-3xl font-semibold md:mb-4 md:text-4xl lg:mb-6 lg:max-w-3xl lg:text-5xl",
                        "Blog Posts"
                    }
                    p {
                        class: "text-muted-foreground mb-8 md:text-base lg:max-w-2xl lg:text-lg",
                        "Discover the latest trends, tips, and best practices in modern web development. From UI components to design systems, stay updated with our expert insights."
                    }
                    a {
                        class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 text-primary underline-offset-4 hover:underline w-full sm:w-auto",
                        href: "https://shadcnblocks.com",
                        target: "_blank",
                        "View all articles"
                        svg { class: "ml-2 size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                    }
                }
                div {
                    class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3 lg:gap-8",
                    // Post 1
                    div {
                        class: "rounded-xl border bg-card text-card-foreground shadow grid grid-rows-[auto_auto_1fr_auto] pt-0",
                        div {
                            class: "aspect-16/9 w-full",
                            a {
                                class: "fade-in transition-opacity duration-200 hover:opacity-70",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                img { class: "h-full w-full object-cover object-center", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg", alt: "Getting Started with shadcn/ui Components" }
                            }
                        }
                        div {
                            class: "flex flex-col space-y-1.5 p-6",
                            h3 {
                                class: "text-lg font-semibold hover:underline md:text-xl",
                                a {
                                    href: "https://shadcnblocks.com",
                                    target: "_blank",
                                    "Getting Started with shadcn/ui Components"
                                }
                            }
                        }
                        div {
                            class: "p-6 pt-0",
                            p {
                                class: "text-muted-foreground",
                                "Learn how to quickly integrate and customize shadcn/ui components in your Next.js projects. We\u{2019}ll cover installation, theming, and best practices for building modern interfaces."
                            }
                        }
                        div {
                            class: "flex items-center p-6 pt-0",
                            a {
                                class: "text-foreground flex items-center hover:underline",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                "Read more"
                                svg { class: "ml-2 size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                            }
                        }
                    }
                    // Post 2
                    div {
                        class: "rounded-xl border bg-card text-card-foreground shadow grid grid-rows-[auto_auto_1fr_auto] pt-0",
                        div {
                            class: "aspect-16/9 w-full",
                            a {
                                class: "fade-in transition-opacity duration-200 hover:opacity-70",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                img { class: "h-full w-full object-cover object-center", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg", alt: "Building Accessible Web Applications" }
                            }
                        }
                        div {
                            class: "flex flex-col space-y-1.5 p-6",
                            h3 {
                                class: "text-lg font-semibold hover:underline md:text-xl",
                                a {
                                    href: "https://shadcnblocks.com",
                                    target: "_blank",
                                    "Building Accessible Web Applications"
                                }
                            }
                        }
                        div {
                            class: "p-6 pt-0",
                            p {
                                class: "text-muted-foreground",
                                "Explore how to create inclusive web experiences using shadcn/ui\u{2019}s accessible components. Discover practical tips for implementing ARIA labels, keyboard navigation, and semantic HTML."
                            }
                        }
                        div {
                            class: "flex items-center p-6 pt-0",
                            a {
                                class: "text-foreground flex items-center hover:underline",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                "Read more"
                                svg { class: "ml-2 size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                            }
                        }
                    }
                    // Post 3
                    div {
                        class: "rounded-xl border bg-card text-card-foreground shadow grid grid-rows-[auto_auto_1fr_auto] pt-0",
                        div {
                            class: "aspect-16/9 w-full",
                            a {
                                class: "fade-in transition-opacity duration-200 hover:opacity-70",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                img { class: "h-full w-full object-cover object-center", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg", alt: "Modern Design Systems with Tailwind CSS" }
                            }
                        }
                        div {
                            class: "flex flex-col space-y-1.5 p-6",
                            h3 {
                                class: "text-lg font-semibold hover:underline md:text-xl",
                                a {
                                    href: "https://shadcnblocks.com",
                                    target: "_blank",
                                    "Modern Design Systems with Tailwind CSS"
                                }
                            }
                        }
                        div {
                            class: "p-6 pt-0",
                            p {
                                class: "text-muted-foreground",
                                "Dive into creating scalable design systems using Tailwind CSS and shadcn/ui. Learn how to maintain consistency while building flexible and maintainable component libraries."
                            }
                        }
                        div {
                            class: "flex items-center p-6 pt-0",
                            a {
                                class: "text-foreground flex items-center hover:underline",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                "Read more"
                                svg { class: "ml-2 size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
