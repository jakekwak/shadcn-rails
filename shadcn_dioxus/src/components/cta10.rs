use dioxus::prelude::*;

#[component]
pub fn Cta10() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "bg-accent flex w-full flex-col gap-16 overflow-hidden rounded-lg p-8 md:rounded-xl lg:flex-row lg:items-center lg:p-12",
                    div {
                        class: "flex-1",
                        h3 {
                            class: "mb-3 text-2xl font-semibold md:mb-4 md:text-4xl lg:mb-6",
                            "Call to Action"
                        }
                        p {
                            class: "text-muted-foreground max-w-xl lg:text-lg",
                            "Build faster with our collection of pre-built blocks. Speed up your development and ship features in record time."
                        }
                    }
                    div {
                        class: "flex shrink-0 flex-col gap-2 sm:flex-row",
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                            href: "https://www.shadcnblocks.com",
                            "Buy Now"
                        }
                    }
                }
            }
        }
    }
}
