use dioxus::prelude::*;
use crate::components::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn Feature51() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                Tabs {
                    class: "p-0",
                    default_value: "tab-1",
                    TabsList {
                        class: "bg-background flex h-auto w-full flex-col gap-2 p-0 md:flex-row",
                        TabsTrigger {
                            class: "hover:border-muted data-[state=active]:bg-muted group flex w-full flex-col items-start justify-start gap-1 whitespace-normal rounded-md border p-4 text-left shadow-none transition-opacity duration-200 hover:opacity-80 data-[state=active]:shadow-none",
                            value: "tab-1",
                            div {
                                class: "flex items-center gap-2 md:flex-col md:items-start lg:gap-4",
                                span { class: "bg-muted text-muted-foreground group-data-[state=active]:bg-primary group-data-[state=active]:text-primary-foreground flex size-8 items-center justify-center rounded-full transition-opacity duration-200 lg:size-10",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" } path { d: "M9 18h6" } path { d: "M10 22h4" } }
                                }
                                p { class: "text-lg font-semibold transition-opacity duration-200 md:text-2xl lg:text-xl", "Research" }
                            }
                            p { class: "text-muted-foreground font-normal transition-opacity duration-200 md:block", "Discover the powerful features that make our platform stand out from the rest." }
                        }
                        TabsTrigger {
                            class: "hover:border-muted data-[state=active]:bg-muted group flex w-full flex-col items-start justify-start gap-1 whitespace-normal rounded-md border p-4 text-left shadow-none transition-opacity duration-200 hover:opacity-80 data-[state=active]:shadow-none",
                            value: "tab-2",
                            div {
                                class: "flex items-center gap-2 md:flex-col md:items-start lg:gap-4",
                                span { class: "bg-muted text-muted-foreground group-data-[state=active]:bg-primary group-data-[state=active]:text-primary-foreground flex size-8 items-center justify-center rounded-full transition-opacity duration-200 lg:size-10",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "m3 17 2 2 4-4" } path { d: "m3 7 2 2 4-4" } path { d: "M13 6h8" } path { d: "M13 12h8" } path { d: "M13 18h8" } }
                                }
                                p { class: "text-lg font-semibold transition-opacity duration-200 md:text-2xl lg:text-xl", "Refine" }
                            }
                            p { class: "text-muted-foreground font-normal transition-opacity duration-200 md:block", "Built with the latest technology and designed for maximum productivity." }
                        }
                        TabsTrigger {
                            class: "hover:border-muted data-[state=active]:bg-muted group flex w-full flex-col items-start justify-start gap-1 whitespace-normal rounded-md border p-4 text-left shadow-none transition-opacity duration-200 hover:opacity-80 data-[state=active]:shadow-none",
                            value: "tab-3",
                            div {
                                class: "flex items-center gap-2 md:flex-col md:items-start lg:gap-4",
                                span { class: "bg-muted text-muted-foreground group-data-[state=active]:bg-primary group-data-[state=active]:text-primary-foreground flex size-8 items-center justify-center rounded-full transition-opacity duration-200 lg:size-10",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" } path { d: "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" } path { d: "M12 2v2" } path { d: "M12 22v-2" } path { d: "m17 20.66-1-1.73" } path { d: "M11 10.27 7 3.34" } path { d: "m20.66 17-1.73-1" } path { d: "m3.34 7 1.73 1" } path { d: "M14 12h8" } path { d: "M2 12h2" } path { d: "m20.66 7-1.73 1" } path { d: "m3.34 17 1.73-1" } path { d: "m17 3.34-1 1.73" } path { d: "m11 13.73-4 6.93" } }
                                }
                                p { class: "text-lg font-semibold transition-opacity duration-200 md:text-2xl lg:text-xl", "Build" }
                            }
                            p { class: "text-muted-foreground font-normal transition-opacity duration-200 md:block", "Create amazing experiences with our comprehensive toolkit and resources." }
                        }
                    }
                    TabsContent {
                        class: "transition-opacity duration-300",
                        value: "tab-1",
                        img { class: "aspect-video w-full rounded-md object-cover transition-opacity duration-300", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg", alt: "Analytics" }
                    }
                    TabsContent {
                        class: "transition-opacity duration-300",
                        value: "tab-2",
                        img { class: "aspect-video w-full rounded-md object-cover transition-opacity duration-300", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-2.svg", alt: "Automation" }
                    }
                    TabsContent {
                        class: "transition-opacity duration-300",
                        value: "tab-3",
                        img { class: "aspect-video w-full rounded-md object-cover transition-opacity duration-300", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-3.svg", alt: "Integration" }
                    }
                }
            }
        }
    }
}
