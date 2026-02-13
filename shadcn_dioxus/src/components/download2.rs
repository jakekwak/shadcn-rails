use dioxus::prelude::*;

#[component]
pub fn Download2() -> Element {
    rsx! {
        section {
            class: "bg-muted/50 py-32",
            div {
                class: "container",
                // Header Section
                div {
                    class: "mb-20 text-center",
                    h2 {
                        class: "mb-6 text-4xl font-bold tracking-tight sm:text-5xl lg:text-6xl",
                        "Available Everywhere"
                    }
                    p {
                        class: "text-muted-foreground mx-auto mb-12 max-w-2xl text-lg",
                        "Choose your platform and start using our app right away. Available on all major devices and operating systems."
                    }
                }

                // Download Options - Minimal Grid
                div {
                    class: "mx-auto grid max-w-4xl gap-12 md:grid-cols-3",
                    // Desktop
                    div {
                        class: "text-center",
                        div {
                            class: "bg-background mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full shadow-sm",
                            // Monitor icon
                            svg { class: "h-10 w-10", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                rect { width: "20", height: "14", x: "2", y: "3", rx: "2" }
                                line { x1: "8", x2: "16", y1: "21", y2: "21" }
                                line { x1: "12", x2: "12", y1: "17", y2: "21" }
                            }
                        }
                        h3 {
                            class: "mb-2 text-xl font-semibold",
                            "PC/Mac"
                        }
                        p {
                            class: "text-muted-foreground mb-6 text-sm",
                            "Complete desktop solution."
                        }
                        a {
                            href: "https://example.com",
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-11 px-8 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                            // Download icon
                            svg { class: "h-4 w-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "7 10 12 15 17 10" }
                                line { x1: "12", x2: "12", y1: "15", y2: "3" }
                            }
                            "Download"
                        }
                    }

                    // iOS
                    div {
                        class: "text-center",
                        div {
                            class: "bg-background mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full shadow-sm",
                            // Smartphone icon
                            svg { class: "h-10 w-10", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                rect { width: "14", height: "20", x: "5", y: "2", rx: "2", ry: "2" }
                                path { d: "M12 18h.01" }
                            }
                        }
                        h3 {
                            class: "mb-2 text-xl font-semibold",
                            "iOS"
                        }
                        p {
                            class: "text-muted-foreground mb-6 text-sm",
                            "Designed specifically for iOS devices."
                        }
                        a {
                            class: "mx-auto block w-fit",
                            href: "https://example.com",
                            img {
                                class: "h-10",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/badges/appstore.png",
                                alt: "Download on the App Store",
                            }
                        }
                    }

                    // Android
                    div {
                        class: "text-center",
                        div {
                            class: "bg-background mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full shadow-sm",
                            // Tablet icon
                            svg { class: "h-10 w-10", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                rect { width: "16", height: "20", x: "4", y: "2", rx: "2", ry: "2" }
                                line { x1: "12", x2: "12.01", y1: "18", y2: "18" }
                            }
                        }
                        h3 {
                            class: "mb-2 text-xl font-semibold",
                            "Android"
                        }
                        p {
                            class: "text-muted-foreground mb-6 text-sm",
                            "Optimized for Android ecosystem."
                        }
                        a {
                            class: "mx-auto block w-fit",
                            href: "https://example.com",
                            img {
                                class: "h-10",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/badges/googleplay.png",
                                alt: "Get it on Google Play",
                            }
                        }
                    }
                }
            }
        }
    }
}
