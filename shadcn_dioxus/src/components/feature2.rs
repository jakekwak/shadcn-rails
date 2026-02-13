use dioxus::prelude::*;

#[component]
pub fn Feature2() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "grid items-center gap-8 md:gap-16 lg:grid-cols-2",
                    img {class: "max-h-96 w-full rounded-md object-cover", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg", alt: "placeholder hero"}
                    div {
                        class: "flex flex-col items-center text-center lg:items-start lg:text-left",
                        h2 {class: "my-6 mt-0 text-balance text-4xl font-semibold lg:text-5xl",
                            "Blocks built with Shadcn & Tailwind"}
                        p {class: "text-muted-foreground mb-8 max-w-xl lg:text-lg",
                            "Hundreds of finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."}
                        div {
                            class: "flex w-full flex-col justify-center gap-2 sm:flex-row lg:justify-start",
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                "Get Started"
                            }
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                                href: "https://shadcnblocks.com",
                                target: "_blank",
                                "Learn More"
                            }
                        }
                    }
                }
            }
        }
    }
}
