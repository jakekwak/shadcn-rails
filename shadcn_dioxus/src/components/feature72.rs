use dioxus::prelude::*;

#[component]
pub fn Feature72() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mb-8 lg:max-w-sm",
                    h2 {class: "mb-3 text-3xl font-semibold md:mb-4 md:text-4xl lg:mb-6",
                        "Powerful Features"}
                    p {class: "text-muted-foreground mb-8 lg:text-lg",
                        "Discover the powerful features that make our platform stand out from the rest. Built with the latest technology and designed for maximum productivity."}
                    a {
                        class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 text-primary underline-offset-4 hover:underline group flex items-center font-medium md:text-base lg:text-lg", href: "https://shadcnblocks.com",
                        "Book a demo"
                        svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M5 12h14"} path {d: "m12 5 7 7-7 7"} }
                    }
                }
                div {
                    class: "grid gap-6 md:grid-cols-2 lg:gap-8",
                    div {
                        class: "border-border flex flex-col overflow-clip rounded-xl border",
                        a {
                            href: "https://shadcnblocks.com",
                            img {class: "aspect-16/9 h-full w-full object-cover object-center transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg", alt: "Modern Design"}
                        }
                        div {
                            class: "px-6 py-8 md:px-8 md:py-10 lg:px-10 lg:py-12",
                            h3 {class: "mb-3 text-lg font-semibold md:mb-4 md:text-2xl lg:mb-6",
                                "Modern Design"}
                            p {class: "text-muted-foreground lg:text-lg",
                                "Clean and intuitive interface built with the latest design principles. Optimized for the best user experience."}
                        }
                    }
                    div {
                        class: "border-border flex flex-col overflow-clip rounded-xl border",
                        a {
                            href: "https://shadcnblocks.com",
                            img {class: "aspect-16/9 h-full w-full object-cover object-center transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-2.svg", alt: "Responsive Layout"}
                        }
                        div {
                            class: "px-6 py-8 md:px-8 md:py-10 lg:px-10 lg:py-12",
                            h3 {class: "mb-3 text-lg font-semibold md:mb-4 md:text-2xl lg:mb-6",
                                "Responsive Layout"}
                            p {class: "text-muted-foreground lg:text-lg",
                                "Fully responsive design that works seamlessly across all devices and screen sizes. Perfect for any platform."}
                        }
                    }
                    div {
                        class: "border-border flex flex-col overflow-clip rounded-xl border",
                        a {
                            href: "https://shadcnblocks.com",
                            img {class: "aspect-16/9 h-full w-full object-cover object-center transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-3.svg", alt: "Easy Integration"}
                        }
                        div {
                            class: "px-6 py-8 md:px-8 md:py-10 lg:px-10 lg:py-12",
                            h3 {class: "mb-3 text-lg font-semibold md:mb-4 md:text-2xl lg:mb-6",
                                "Easy Integration"}
                            p {class: "text-muted-foreground lg:text-lg",
                                "Simple integration process with comprehensive documentation and dedicated support team."}
                        }
                    }
                    div {
                        class: "border-border flex flex-col overflow-clip rounded-xl border",
                        a {
                            href: "https://shadcnblocks.com",
                            img {class: "aspect-16/9 h-full w-full object-cover object-center transition-opacity hover:opacity-80", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-4.svg", alt: "Advanced Analytics"}
                        }
                        div {
                            class: "px-6 py-8 md:px-8 md:py-10 lg:px-10 lg:py-12",
                            h3 {class: "mb-3 text-lg font-semibold md:mb-4 md:text-2xl lg:mb-6",
                                "Advanced Analytics"}
                            p {class: "text-muted-foreground lg:text-lg",
                                "Powerful analytics tools to help you understand your users and make data-driven decisions."}
                        }
                    }
                }
            }
        }
    }
}
