use dioxus::prelude::*;
use crate::components::Separator;

#[component]
pub fn Timeline9() -> Element {
    rsx! {
        section {
            class: "bg-background py-32",
            div {
                class: "container",
                h1 {
                    class: "text-foreground mb-10 text-center text-3xl font-bold tracking-tighter sm:text-6xl",
                    "The History of Artificial Intelligence"
                }
                div {
                    class: "relative mx-auto max-w-4xl",
                    Separator {
                        class: "bg-muted absolute left-2 top-4",
                        orientation: "vertical",
                    }
                    div {
                        class: "relative mb-10 pl-8",
                        div { class: "bg-foreground absolute left-0 top-3.5 flex size-4 items-center justify-center rounded-full" }
                        h4 { class: "rounded-xl py-2 text-xl font-bold tracking-tight xl:mb-4 xl:px-3", "The Birth of AI" }
                        h5 { class: "text-md -left-34 text-muted-foreground top-3 rounded-xl tracking-tight xl:absolute", "1956" }
                        div {
                            class: "rounded-xl border bg-card text-card-foreground shadow my-5 border-none shadow-none",
                            div {
                                class: "p-6 pt-0 px-0 xl:px-2",
                                p { "The term 'Artificial Intelligence' was coined at the Dartmouth Conference, marking the official beginning of AI as a field. John McCarthy, Marvin Minsky, Nathaniel Rochester, and Claude Shannon organized this seminal event, setting the stage for decades of research and development." }
                            }
                        }
                    }
                    div {
                        class: "relative mb-10 pl-8",
                        div { class: "bg-foreground absolute left-0 top-3.5 flex size-4 items-center justify-center rounded-full" }
                        h4 { class: "rounded-xl py-2 text-xl font-bold tracking-tight xl:mb-4 xl:px-3", "Early Optimism and First AI Winter" }
                        h5 { class: "text-md -left-34 text-muted-foreground top-3 rounded-xl tracking-tight xl:absolute", "1966-1973" }
                        div {
                            class: "rounded-xl border bg-card text-card-foreground shadow my-5 border-none shadow-none",
                            div {
                                class: "p-6 pt-0 px-0 xl:px-2",
                                p { "The early years saw significant optimism with programs like ELIZA (the first chatbot) and SHRDLU (a natural language understanding system). However, by the early 1970s, funding dried up as researchers faced the limitations of early computing power and the complexity of human intelligence." }
                            }
                        }
                    }
                    div {
                        class: "relative mb-10 pl-8",
                        div { class: "bg-foreground absolute left-0 top-3.5 flex size-4 items-center justify-center rounded-full" }
                        h4 { class: "rounded-xl py-2 text-xl font-bold tracking-tight xl:mb-4 xl:px-3", "Expert Systems and Revival" }
                        h5 { class: "text-md -left-34 text-muted-foreground top-3 rounded-xl tracking-tight xl:absolute", "1980-1987" }
                        div {
                            class: "rounded-xl border bg-card text-card-foreground shadow my-5 border-none shadow-none",
                            div {
                                class: "p-6 pt-0 px-0 xl:px-2",
                                p { "AI experienced a revival with the development of expert systems like MYCIN (for medical diagnosis) and DENDRAL (for chemical analysis). These systems used rule-based approaches to mimic human decision-making in specific domains, leading to renewed interest and funding in AI research." }
                            }
                        }
                    }
                    div {
                        class: "relative mb-10 pl-8",
                        div { class: "bg-foreground absolute left-0 top-3.5 flex size-4 items-center justify-center rounded-full" }
                        h4 { class: "rounded-xl py-2 text-xl font-bold tracking-tight xl:mb-4 xl:px-3", "Deep Blue Defeats Chess Champion" }
                        h5 { class: "text-md -left-34 text-muted-foreground top-3 rounded-xl tracking-tight xl:absolute", "1997" }
                        div {
                            class: "rounded-xl border bg-card text-card-foreground shadow my-5 border-none shadow-none",
                            div {
                                class: "p-6 pt-0 px-0 xl:px-2",
                                p { "IBM\u{2019}s Deep Blue became the first computer system to defeat a reigning world chess champion, Garry Kasparov, in a six-game match. This milestone demonstrated AI\u{2019}s potential to outperform humans in complex strategic games and captured the public\u{2019}s imagination." }
                            }
                        }
                    }
                }
            }
        }
    }
}
