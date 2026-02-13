use dioxus::prelude::*;
use crate::components::{Button};

#[component]
pub fn Banner1() -> Element {
    rsx! {
        section {
            class: "bg-background w-full border-b px-4 py-3",
            div {
                class: "flex items-center justify-between gap-2",
                div {
                    class: "flex-1 text-center",
                    span {
                        class: "text-sm",
                        span {
                            class: "font-medium",
                            "Version 2.0 is now available! "
                        }
                        span {
                            class: "text-muted-foreground",
                            "Read the full release notes "
                            a {
                                class: "hover:text-foreground underline underline-offset-2",
                                href: "https://example.com",
                                target: "_blank",
                                "here"
                            }
                            "."
                        }
                    }
                }

                Button {
                    class: "-mr-2 h-8 w-8 flex-none",
                    variant: "ghost",
                    size: "icon",
                    svg {
                        class: "size-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M18 6 6 18" }
                        path { d: "m6 6 12 12" }
                    }
                }
            }
        }
    }
}
