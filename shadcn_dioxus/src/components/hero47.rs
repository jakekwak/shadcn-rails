use dioxus::prelude::*;

#[component]
pub fn Hero47() -> Element {
    rsx! {
        section {
            class: "bg-background py-20 lg:py-32",
            div {
                class: "container flex flex-col items-center gap-10 lg:my-0 lg:flex-row",
                div {
                    class: "flex flex-col gap-7 lg:w-2/3",
                    h2 {
                        class: "text-foreground text-5xl font-semibold md:text-5xl lg:text-8xl",
                        span { "Epic Blocks" }
                        span {
                            class: "text-muted-foreground",
                            " built with shadcn/ui & Tailwind"
                        }
                    }
                    p {
                        class: "text-muted-foreground text-base md:text-lg lg:text-xl",
                        "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."
                    }
                    div {
                        class: "flex flex-wrap items-start gap-5 lg:gap-7",
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                            href: "#",
                            div {
                                class: "flex items-center gap-2",
                                svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M7 7h10v10" } path { d: "M7 17 17 7" } }
                            }
                            span {
                                class: "whitespace-nowrap pl-4 pr-6 text-sm lg:pl-6 lg:pr-8 lg:text-base",
                                "Get Started"
                            }
                        }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 text-primary underline-offset-4 hover:underline underline",
                            href: "#",
                            "Read the docs"
                        }
                    }
                }
                div {
                    class: "relative z-10",
                    div {
                        class: "absolute top-2.5 left-1/2 h-[92%] w-[69%] -translate-x-[52%] overflow-hidden rounded-[35px]",
                        img {
                            class: "size-full object-cover object-[50%_0%]",
                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-dark-7-tall.svg",
                            alt: "Placeholder",
                        }
                    }
                    img {
                        class: "relative z-10",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/mockups/phone-2.png",
                        alt: "iphone",
                        width: "450",
                        height: "889",
                    }
                }
            }
        }
    }
}
