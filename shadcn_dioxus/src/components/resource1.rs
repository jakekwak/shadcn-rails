use dioxus::prelude::*;
use crate::components::{Button};

#[component]
pub fn Resource1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container grid gap-12 md:grid-cols-12 md:gap-8",
                div {
                    class: "order-last md:order-none md:col-span-4 lg:col-span-3",
                    aside {
                        class: "flex flex-col gap-2",
                        div {
                            class: "border-border bg-card mb-6 overflow-hidden rounded-lg border shadow-sm",
                            div {
                                class: "border-border bg-muted/50 border-b px-5 py-4",
                                h3 {class: "flex items-center gap-2 text-sm font-semibold",
                                    // Book icon
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" }
                                    }
                                    "Whitepaper"}
                            }
                            div {
                                class: "p-5",
                                div {
                                    class: "text-foreground gap-4 text-lg font-semibold leading-snug",
                                    p {"The Complete Guide to Launching Your Startup"}
                                }
                            }
                        }

                        div {
                            class: "border-border bg-card mb-6 overflow-hidden rounded-lg border shadow-sm",
                            div {
                                class: "border-border bg-muted/50 border-b px-5 py-4",
                                h3 {
                                    class: "flex items-center text-sm font-semibold",
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"} }
                                    "Download Options"
                                }
                            }
                            div {
                                class: "p-5",
                                div {
                                    class: "space-y-4",
                                    p {class: "text-muted-foreground text-sm",
                                        "Enjoy this guide? Download it for offline reading or sharing."}
                                    div {
                                        class: "flex flex-col space-y-2",
                                        Button {
                                            class: "w-full justify-between", variant: "default",
                                            "PDF Format"
                                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"} }
                                        }
                                        Button {
                                            class: "w-full justify-between", variant: "outline",
                                            "Print Version"
                                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"} }
                                        }
                                    }
                                    p {class: "text-muted-foreground mt-4 text-center text-xs",
                                        "Read time: 5 minutes"}
                                }
                            }
                        }

                        div {
                            class: "border-border bg-card mb-6 overflow-hidden rounded-lg border shadow-sm",
                            div {
                                class: "border-border bg-muted/50 border-b px-5 py-4",
                                h3 {class: "flex items-center gap-2 text-sm font-semibold",
                                    // Share2 icon
                                    svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        circle { cx: "18", cy: "5", r: "3" }
                                        circle { cx: "6", cy: "12", r: "3" }
                                        circle { cx: "18", cy: "19", r: "3" }
                                        line { x1: "8.59", x2: "15.42", y1: "13.51", y2: "17.49" }
                                        line { x1: "15.41", x2: "8.59", y1: "6.51", y2: "10.49" }
                                    }
                                    "Share this guide"}
                            }
                            div {
                                class: "p-5",
                                ul {
                                    class: "flex items-center gap-2",
                                    li {
                                        a {
                                            class: "border-border bg-muted/50 hover:bg-muted flex size-10 items-center justify-center rounded-full border transition-colors", href: "https://example.com", aria_label: "Share on Instagram",
                                            img {class: "size-5", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/instagram-icon.svg", alt: "Instagram"}
                                        }
                                    }
                                    li {
                                        a {
                                            class: "border-border bg-muted/50 hover:bg-muted flex size-10 items-center justify-center rounded-full border transition-colors", href: "https://example.com", aria_label: "Share on LinkedIn",
                                            img {class: "size-5", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/linkedin-icon.svg", alt: "LinkedIn"}
                                        }
                                    }
                                    li {
                                        a {
                                            class: "border-border bg-muted/50 hover:bg-muted flex size-10 items-center justify-center rounded-full border transition-colors", href: "https://example.com", aria_label: "Share on Product Hunt",
                                            img {class: "size-5", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/producthunt-icon.svg", alt: "Product Hunt"}
                                        }
                                    }
                                    li {
                                        a {
                                            class: "border-border bg-muted/50 hover:bg-muted flex size-10 items-center justify-center rounded-full border transition-colors", href: "https://example.com", aria_label: "Share on Twitter",
                                            img {class: "size-5", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/twitter-icon.svg", alt: "Twitter"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "md:col-span-7 md:col-start-5 lg:col-start-6",
                    article {
                        class: "prose prose-sm",
                        h1 { "White Paper: The Complete Guide to Launching Your Startup" }
                        p { "Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne. One day, his advisors came to him with a problem: the kingdom was running out of money." }
                        h2 { "The King's Plan" }
                        p {
                            "The king thought long and hard, and finally came up with "
                            a {
                                href: "https://example.com",
                                "a brilliant plan"
                            }
                            ": he would tax the jokes in the kingdom."
                        }
                        blockquote { "After all, he said, everyone enjoys a good joke, so it is only fair that they should pay for the privilege." }
                        h3 { "The Joke Tax" }
                        p { "The king's subjects were not amused. They grumbled and complained, but the king was firm:" }
                        ul {
                            li { "1st level of puns: 5 gold coins" }
                            li { "2nd level of jokes: 10 gold coins" }
                            li { "3rd level of one-liners: 20 gold coins" }
                        }
                        p { "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king's foolishness get him down: a court jester named Jokester." }
                        h3 { "Jokester's Revolt" }
                        p { "Jokester began sneaking into the castle in the middle of the night and leaving jokes all over the place: under the king's pillow, in his soup, even in the royal toilet. The king was furious, but he couldn't seem to stop Jokester." }
                        p { "And then, one day, the people of the kingdom discovered that the jokes left by Jokester were so funny that they couldn't help but laugh. And once they started laughing, they couldn't stop." }
                        h3 { "The People's Rebellion" }
                        p { "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke." }
                        div {
                            table {
                                thead {
                                    tr {
                                        th {"King's Treasury"}
                                        th {"People's happiness"}
                                    }
                                }
                                tbody {
                                    tr {
                                        td {"Empty"}
                                        td {"Overflowing"}
                                    }
                                    tr {
                                        class: "even:bg-muted m-0 border-t p-0",
                                        td {"Modest"}
                                        td {"Satisfied"}
                                    }
                                    tr {
                                        class: "even:bg-muted m-0 border-t p-0",
                                        td {"Full"}
                                        td {"Ecstatic"}
                                    }
                                }
                            }
                        }
                        p {"The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after."}
                        p {"The moral of the story is: never underestimate the power of a good laugh and always be careful of bad ideas."}
                    }
                }
            }
        }
    }
}
