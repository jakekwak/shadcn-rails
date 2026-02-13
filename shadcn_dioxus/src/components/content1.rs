use dioxus::prelude::*;
use crate::components::{Alert, AlertTitle, AlertDescription, Badge};

#[component]
pub fn Content1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container max-w-7xl",
                div {
                    class: "relative grid-cols-3 gap-20 lg:grid",
                    // Main content (2 cols)
                    div {
                        class: "lg:col-span-2",
                        // Header
                        div {
                            Badge { variant: "outline", "Kingdom Tales" }
                            h1 { class: "mt-3 text-3xl font-extrabold",
                                "The Great Joke Tax"
                            }
                            p { class: "text-muted-foreground mt-2 text-lg",
                                "In a kingdom far away, where laughter once flowed freely, a peculiar tale unfolded about a king who decided to tax the very essence of joy itself - jokes and jest."
                            }
                            img {
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                                alt: "placeholder",
                                class: "my-8 aspect-video w-full rounded-md object-cover",
                            }
                        }

                        // Section 1: How the Tax System Works
                        section {
                            id: "section1",
                            class: "prose mb-8",
                            h2 { "How the Tax System Works" }
                            div {
                                class: "ml-3.5",
                                // Step 1
                                div {
                                    class: "relative flex items-start pb-2",
                                    div { class: "bg-border/70 absolute top-[2.75rem] h-[calc(100%-2.75rem)] w-px" }
                                    div {
                                        class: "absolute ml-[-14px] py-2",
                                        div {
                                            class: "bg-muted flex size-7 shrink-0 items-center justify-center rounded-lg",
                                            // RefreshCcw icon
                                            svg { class: "h-3.5 w-3.5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                                path { d: "M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                                                path { d: "M3 3v5h5" }
                                                path { d: "M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" }
                                                path { d: "M16 16h5v5" }
                                            }
                                        }
                                    }
                                    div {
                                        class: "pl-12",
                                        h3 { class: "mt-2 text-base font-semibold", "Registering Your Jokes" }
                                        p { "All citizens must register their jokes at the Royal Jest Office. Each joke is carefully cataloged and assigned a tax bracket based on its humor level." }
                                    }
                                }
                                // Step 2
                                div {
                                    class: "relative flex items-start pb-2",
                                    div { class: "bg-border/70 absolute top-[2.75rem] h-[calc(100%-2.75rem)] w-px" }
                                    div {
                                        class: "absolute ml-[-14px] py-2",
                                        div {
                                            class: "bg-muted flex size-7 shrink-0 items-center justify-center rounded-lg",
                                            // GalleryVerticalEnd icon
                                            svg { class: "h-3.5 w-3.5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                                path { d: "M7 2h10" }
                                                path { d: "M5 6h14" }
                                                rect { x: "3", y: "10", width: "18", height: "12", rx: "2" }
                                            }
                                        }
                                    }
                                    div {
                                        class: "pl-12",
                                        h3 { class: "mt-2 text-base font-semibold", "Classification Process" }
                                        p { "The Royal Jesters evaluate each joke based on wit, delivery, and audience reaction. Higher ratings mean higher taxes, making the finest jokes a luxury few can afford." }
                                    }
                                }
                                // Step 3
                                div {
                                    class: "relative flex items-start pb-2",
                                    div { class: "bg-border/70 absolute top-[2.75rem] h-[calc(100%-2.75rem)] w-px" }
                                    div {
                                        class: "absolute ml-[-14px] py-2",
                                        div {
                                            class: "bg-muted flex size-7 shrink-0 items-center justify-center rounded-lg",
                                            // ListChecks icon
                                            svg { class: "h-3.5 w-3.5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                                path { d: "m3 17 2 2 4-4" }
                                                path { d: "m3 7 2 2 4-4" }
                                                path { d: "M13 6h8" }
                                                path { d: "M13 12h8" }
                                                path { d: "M13 18h8" }
                                            }
                                        }
                                    }
                                    div {
                                        class: "pl-12",
                                        h3 { class: "mt-2 text-base font-semibold", "Tax Collection" }
                                        p { "Royal tax collectors roam the streets, listening for laughter. Anyone caught telling an unregistered joke faces steep fines or time in the kingdom\u{2019}s least amusing dungeon." }
                                    }
                                }
                            }
                        }

                        // Section 2: The People's Rebellion
                        section {
                            id: "section2",
                            class: "prose mb-8",
                            h2 { "The People\u{2019}s Rebellion" }
                            p { "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke." }
                            div {
                                table {
                                    thead {
                                        tr {
                                            th { "King\u{2019}s Treasury" }
                                            th { "People\u{2019}s happiness" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            td { "Empty" }
                                            td { "Overflowing" }
                                        }
                                        tr {
                                            class: "even:bg-muted m-0 border-t p-0",
                                            td { "Modest" }
                                            td { "Satisfied" }
                                        }
                                        tr {
                                            class: "even:bg-muted m-0 border-t p-0",
                                            td { "Full" }
                                            td { "Ecstatic" }
                                        }
                                    }
                                }
                            }
                            p { "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after." }
                            Alert {
                                // Lightbulb icon
                                svg { class: "h-4 w-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                    path { d: "M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" }
                                    path { d: "M9 18h6" }
                                    path { d: "M10 22h4" }
                                }
                                AlertTitle { "Royal Decree!" }
                                AlertDescription { "Remember, all jokes must be registered at the Royal Jest Office before telling them" }
                            }
                        }

                        // Section 3: The King's Plan
                        section {
                            id: "section3",
                            class: "prose mb-8",
                            h2 { "The King\u{2019}s Plan" }
                            p {
                                "The king thought long and hard, and finally came up with "
                                a { href: "https://example.com", "a brilliant plan" }
                                ": he would tax the jokes in the kingdom."
                            }
                            blockquote {
                                "\u{201C}After all,\u{201D} he said, \u{201C}everyone enjoys a good joke, so it\u{2019}s only fair that they should pay for the privilege.\u{201D}"
                            }
                            p { "The king\u{2019}s subjects were not amused. They grumbled and complained, but the king was firm:" }
                            ul {
                                li { "1st level of puns: 5 gold coins" }
                                li { "2nd level of jokes: 10 gold coins" }
                                li { "3rd level of one-liners : 20 gold coins" }
                            }
                            p { "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king\u{2019}s foolishness get him down: a court jester named Jokester." }
                        }
                    }

                    // Sidebar: On this page
                    div {
                        class: "sticky top-8 hidden h-fit lg:block",
                        span {
                            class: "flex items-center gap-2 text-sm",
                            // AlignLeft icon
                            svg { class: "h-4 w-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M15 12H3" }
                                path { d: "M17 18H3" }
                                path { d: "M21 6H3" }
                            }
                            "On this page"
                        }
                        nav {
                            class: "mt-2 text-sm",
                            ul {
                                li {
                                    a {
                                        href: "#section1",
                                        class: "block py-1 transition-colors duration-200 text-muted-foreground hover:text-primary",
                                        "How the Tax System Works"
                                    }
                                }
                                li {
                                    a {
                                        href: "#section2",
                                        class: "block py-1 transition-colors duration-200 text-muted-foreground hover:text-primary",
                                        "The People\u{2019}s Rebellion"
                                    }
                                }
                                li {
                                    a {
                                        href: "#section3",
                                        class: "block py-1 transition-colors duration-200 text-muted-foreground hover:text-primary",
                                        "The King\u{2019}s Plan"
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
