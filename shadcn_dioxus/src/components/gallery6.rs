use dioxus::prelude::*;

struct GalleryItem {
    title: &'static str,
    summary: &'static str,
    url: &'static str,
    image: &'static str,
}

const ITEMS: &[GalleryItem] = &[
    GalleryItem {
        title: "Build Modern UIs",
        summary: "Create stunning user interfaces with our comprehensive design system.",
        url: "https://example.com",
        image: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg",
    },
    GalleryItem {
        title: "Computer Vision Technology",
        summary: "Powerful image recognition and processing capabilities that allow AI systems to analyze, understand, and interpret visual information from the world.",
        url: "https://example.com",
        image: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg",
    },
    GalleryItem {
        title: "Machine Learning Automation",
        summary: "Self-improving algorithms that learn from data patterns to automate complex tasks and make intelligent decisions with minimal human intervention.",
        url: "https://example.com",
        image: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg",
    },
    GalleryItem {
        title: "Predictive Analytics",
        summary: "Advanced forecasting capabilities that analyze historical data to predict future trends and outcomes, helping businesses make data-driven decisions.",
        url: "https://example.com",
        image: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg",
    },
    GalleryItem {
        title: "Neural Network Architecture",
        summary: "Sophisticated AI models inspired by human brain structure, capable of solving complex problems through deep learning and pattern recognition.",
        url: "https://example.com",
        image: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-1.svg",
    },
];

#[component]
pub fn Gallery6() -> Element {
    let mut offset = use_signal(|| 0i32);

    let can_prev = offset() > 0;
    let can_next = offset() < (ITEMS.len() as i32 - 1).max(0);

    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mb-8 flex flex-col justify-between md:mb-14 md:flex-row md:items-end lg:mb-16",
                    div {
                        h2 {
                            class: "mb-3 text-3xl font-semibold md:mb-4 md:text-4xl lg:mb-6",
                            "Gallery"
                        }
                        a {
                            href: "https://www.shadcnblocks.com",
                            class: "group flex items-center gap-1 text-sm font-medium md:text-base lg:text-lg",
                            "Book a demo"
                            // ArrowUpRight
                            svg { class: "size-4 transition-transform group-hover:translate-x-1", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M7 7h10v10" }
                                path { d: "M7 17 17 7" }
                            }
                        }
                    }
                    div {
                        class: "mt-8 flex shrink-0 items-center justify-start gap-2",
                        // Prev button
                        button {
                            class: "inline-flex items-center justify-center size-10 rounded-md border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground disabled:pointer-events-auto disabled:opacity-50",
                            disabled: !can_prev,
                            onclick: move |_| { if can_prev { offset -= 1; } },
                            // ArrowLeft
                            svg { class: "size-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "m12 19-7-7 7-7" }
                                path { d: "M19 12H5" }
                            }
                        }
                        // Next button
                        button {
                            class: "inline-flex items-center justify-center size-10 rounded-md border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground disabled:pointer-events-auto disabled:opacity-50",
                            disabled: !can_next,
                            onclick: move |_| { if can_next { offset += 1; } },
                            // ArrowRight
                            svg { class: "size-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M5 12h14" }
                                path { d: "m12 5 7 7-7 7" }
                            }
                        }
                    }
                }
            }
            div {
                class: "w-full max-w-full overflow-hidden",
                div {
                    class: "flex transition-transform duration-300 ease-in-out gap-8 pl-8",
                    style: "transform: translateX(-{offset() as f64 * 484.0}px)",
                    for item in ITEMS.iter() {
                        div {
                            class: "min-w-0 shrink-0 grow-0 basis-[calc(100%-4rem)] md:basis-[452px]",
                            a {
                                href: "{item.url}",
                                class: "group flex flex-col justify-between",
                                div {
                                    div {
                                        class: "aspect-[3/2] flex overflow-clip rounded-xl",
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "relative h-full w-full origin-bottom transition duration-300 group-hover:scale-105",
                                                img {
                                                    src: "{item.image}",
                                                    alt: "{item.title}",
                                                    class: "h-full w-full object-cover object-center",
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "mb-2 line-clamp-3 break-words pt-4 text-lg font-medium md:mb-3 md:pt-4 md:text-xl lg:pt-4 lg:text-2xl",
                                    "{item.title}"
                                }
                                div {
                                    class: "text-muted-foreground mb-8 line-clamp-2 text-sm md:mb-12 md:text-base lg:mb-9",
                                    "{item.summary}"
                                }
                                div {
                                    class: "flex items-center text-sm",
                                    "Read more"
                                    " "
                                    // ArrowRight
                                    svg { class: "ml-2 size-5 transition-transform group-hover:translate-x-1", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        path { d: "M5 12h14" }
                                        path { d: "m12 5 7 7-7 7" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
