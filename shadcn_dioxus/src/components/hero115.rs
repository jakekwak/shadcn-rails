use dioxus::prelude::*;

#[component]
pub fn Hero115() -> Element {
    rsx! {
        section {
            class: "overflow-hidden py-32",
            div {
                class: "container",
                div {
                    class: "flex flex-col gap-5",
                    div {
                        class: "relative flex flex-col gap-5",
                        div {
                            class: "absolute left-1/2 top-1/2 -z-10 mx-auto size-[800px] rounded-full border p-16 [mask-image:linear-gradient(to_top,transparent,transparent,white,white,white,transparent,transparent)] md:size-[1300px] md:p-32",
                            style: "transform: translate(-50%, -50%)",
                            div {
                                class: "size-full rounded-full border p-16 md:p-32",
                                div { class: "size-full rounded-full border" }
                            }
                        }
                        span {
                            class: "mx-auto flex size-16 items-center justify-center rounded-full border md:size-20",
                            // Wifi icon
                            svg { class: "size-6", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M12 20h.01" }
                                path { d: "M2 8.82a15 15 0 0 1 20 0" }
                                path { d: "M5 12.859a10 10 0 0 1 14 0" }
                                path { d: "M8.5 16.429a5 5 0 0 1 7 0" }
                            }
                        }
                        h2 {
                            class: "mx-auto max-w-5xl text-balance text-center text-3xl font-medium md:text-6xl",
                            "Blocks built with Shadcn & Tailwind"
                        }
                        p {
                            class: "text-muted-foreground mx-auto max-w-3xl text-center md:text-lg",
                            "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."
                        }
                        div {
                            class: "flex flex-col items-center justify-center gap-3 pb-12 pt-3",
                            a {
                                class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                                href: "https://www.shadcnblocks.com",
                                "Discover Features"
                                // Zap icon
                                svg { class: "ml-2 size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                    path { d: "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" }
                                }
                            }
                            div {
                                class: "text-muted-foreground text-xs",
                                "Trusted by 25.000+ Businesses Worldwide"
                            }
                        }
                    }
                    img {
                        class: "mx-auto h-full max-h-[524px] w-full max-w-5xl rounded-2xl object-cover",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        alt: "placeholder",
                    }
                }
            }
        }
    }
}
