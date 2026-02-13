use dioxus::prelude::*;

#[component]
pub fn About3() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mb-14 grid gap-5 text-center md:grid-cols-2 md:text-left",
                    h1 {
                        class: "text-5xl font-semibold",
                        "About Us"
                    }
                    p {
                        class: "text-muted-foreground",
                        "Shadcnblocks is a passionate team dedicated to creating innovative solutions that empower businesses to thrive in the digital age."
                    }
                }
                div {
                    class: "grid gap-7 lg:grid-cols-3",
                    img {
                        class: "size-full max-h-[620px] rounded-xl object-cover lg:col-span-2",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        alt: "placeholder",
                    }
                    div {
                        class: "flex flex-col gap-7 md:flex-row lg:flex-col",
                        div {
                            class: "bg-muted flex flex-col justify-between gap-6 rounded-xl p-7 md:w-1/2 lg:w-auto",
                            img {
                                class: "mr-auto h-12",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/block-1.svg",
                                alt: "logo",
                            }
                            div {
                                p {
                                    class: "mb-2 text-lg font-semibold",
                                    "Hundreds of blocks at Shadcnblocks.com"
                                }
                                p {
                                    class: "text-muted-foreground",
                                    "Providing businesses with effective tools to improve workflows, boost efficiency, and encourage growth."
                                }
                            }
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground mr-auto",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                "Discover more"
                            }
                        }
                        img {
                            class: "grow basis-0 rounded-xl object-cover md:w-1/2 lg:min-h-0 lg:w-auto",
                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-2.svg",
                            alt: "placeholder",
                        }
                    }
                }
                div {
                    class: "py-32",
                    p {
                        class: "text-center",
                        "Valued by clients worldwide"
                    }
                    div {
                        class: "mt-8 flex flex-wrap justify-center gap-8",
                        div {
                            class: "flex items-center gap-3",
                            img { class: "h-6 w-auto md:h-8", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/astro-wordmark.svg", alt: "Arc" }
                        }
                        div {
                            class: "flex items-center gap-3",
                            img { class: "h-6 w-auto md:h-8", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/vercel-wordmark.svg", alt: "Descript" }
                        }
                        div {
                            class: "flex items-center gap-3",
                            img { class: "h-6 w-auto md:h-8", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/figma-wordmark.svg", alt: "Mercury" }
                        }
                        div {
                            class: "flex items-center gap-3",
                            img { class: "h-6 w-auto md:h-8", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/supabase-wordmark.svg", alt: "Ramp" }
                        }
                        div {
                            class: "flex items-center gap-3",
                            img { class: "h-6 w-auto md:h-8", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/tailwind-wordmark.svg", alt: "Retool" }
                        }
                        div {
                            class: "flex items-center gap-3",
                            img { class: "h-6 w-auto md:h-8", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/astro-wordmark.svg", alt: "Watershed" }
                        }
                    }
                }
                div {
                    class: "bg-muted relative overflow-hidden rounded-xl p-10 md:p-16",
                    div {
                        class: "flex flex-col gap-4 text-center md:text-left",
                        h2 {
                            class: "text-4xl font-semibold",
                            "Our Achievements in Numbers"
                        }
                        p {
                            class: "text-muted-foreground max-w-xl",
                            "Providing businesses with effective tools to improve workflows, boost efficiency, and encourage growth."
                        }
                    }
                    div {
                        class: "mt-10 flex flex-wrap justify-between gap-10 text-center",
                        div {
                            class: "flex flex-col gap-4",
                            p { "Companies Supported" }
                            span { class: "text-4xl font-semibold md:text-5xl", "300+" }
                        }
                        div {
                            class: "flex flex-col gap-4",
                            p { "Projects Finalized" }
                            span { class: "text-4xl font-semibold md:text-5xl", "800+" }
                        }
                        div {
                            class: "flex flex-col gap-4",
                            p { "Happy Customers" }
                            span { class: "text-4xl font-semibold md:text-5xl", "99%" }
                        }
                        div {
                            class: "flex flex-col gap-4",
                            p { "Recognized Awards" }
                            span { class: "text-4xl font-semibold md:text-5xl", "10+" }
                        }
                    }
                    div {
                        class: "pointer-events-none absolute -top-1 right-1 z-10 hidden h-full w-full bg-[linear-gradient(to_right,hsl(var(--muted-foreground))_1px,transparent_1px),linear-gradient(to_bottom,hsl(var(--muted-foreground))_1px,transparent_1px)] bg-[size:80px_80px] opacity-15 [mask-image:linear-gradient(to_bottom_right,#000,transparent,transparent)] md:block",
                    }
                }
            }
        }
    }
}
