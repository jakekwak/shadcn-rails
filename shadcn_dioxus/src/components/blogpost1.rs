use dioxus::prelude::*;
use crate::components::{Alert, AlertDescription, AlertTitle, Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn Blogpost1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto flex max-w-5xl flex-col items-center gap-4 text-center",
                    h1 {
                        class: "max-w-3xl text-pretty text-5xl font-semibold md:text-6xl",
                        "Designing websites faster with shadcn/ui"
                    }
                    h3 {
                        class: "text-muted-foreground max-w-3xl text-lg md:text-xl",
                        "A step-by-step guide to building a modern, responsive blog using React and Tailwind CSS."
                    }
                    div {
                        class: "flex items-center gap-3 text-sm md:text-base",
                        Avatar {
                            class: "h-8 w-8 border",
                            AvatarImage { src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/avatar-2.webp" }
                            AvatarFallback { "JD" }
                        }
                        span {
                            a {
                                class: "font-semibold",
                                href: "https://example.com",
                                "John Doe"
                            }
                            span {
                                class: "ml-1",
                                "on January 1, 2024"
                            }
                        }
                    }
                    img {
                        class: "mb-8 mt-4 aspect-video w-full rounded-lg border object-cover",
                        alt: "placeholder",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                    }
                }
            }
            div {
                class: "container",
                article {
                    class: "prose mx-auto max-w-3xl",
                    h2 {
                        class: "text-3xl font-extrabold",
                        "The Great Joke Tax"
                    }
                    p {
                        class: "text-muted-foreground mt-2 text-lg",
                        "In a kingdom far away, where laughter once flowed freely, a peculiar tale unfolded about a king who decided to tax the very essence of joy itself — jokes and jest."
                    }
                    h2 { "How the Tax System Works" }
                    p { "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after." }
                    Alert {
                        AlertTitle { "Royal Decree!" }
                        AlertDescription { "Remember, all jokes must be registered at the Royal Jest Office before telling them." }
                    }
                    h2 { "The People\u{2019}s Rebellion" }
                    p { "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke." }
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
                    p { "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after." }
                    h2 { "The King\u{2019}s Plan" }
                    img {
                        class: "my-8 aspect-video w-full rounded-md object-cover",
                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        alt: "placeholder",
                    }
                    p {
                        "The king thought long and hard, and finally came up with "
                        a {
                            href: "https://example.com",
                            "a brilliant plan"
                        }
                        ": he would tax the jokes in the kingdom."
                    }
                    blockquote {
                        "\u{201C}After all,\u{201D} he said, \u{201C}everyone enjoys a good joke, so it\u{2019}s only fair that they should pay for the privilege.\u{201D}"
                    }
                    p { "The king\u{2019}s subjects were not amused. They grumbled and complained, but the king was firm:" }
                    ul {
                        li { "1st level of puns: 5 gold coins" }
                        li { "2nd level of jokes: 10 gold coins" }
                        li { "3rd level of one-liners: 20 gold coins" }
                    }
                    p { "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king\u{2019}s foolishness get him down: a court jester named Jokester." }
                }
            }
        }
    }
}
