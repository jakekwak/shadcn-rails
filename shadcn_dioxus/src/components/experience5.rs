use dioxus::prelude::*;

#[component]
pub fn Experience5() -> Element {
    rsx! {
        section {
            class: "py-16 md:py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto max-w-7xl",
                    h2 {class: "mb-12 font-serif text-4xl font-medium leading-tight md:text-7xl",
                        "Work Experience"}
                    div {
                        class: "space-y-8",
                        // Senior Full Stack Developer
                        div {
                            class: "border-border border-b pb-6 last:border-b-0",
                            div {
                                class: "flex flex-col gap-4 md:flex-row md:items-start",
                                div {
                                    class: "md:w-2/3",
                                    div {
                                        class: "mb-2 flex items-center gap-3",
                                        img { class: "h-5 object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/claude-icon.svg", alt: "Anthropic logo" }
                                        h3 { class: "text-xl", "Senior Full Stack Developer" }
                                    }
                                    p { class: "text-muted-foreground mb-3 text-sm", "Full-Time \u{2022} Remote \u{2022} San Francisco CA" }
                                    p { class: "text-muted-foreground text-sm leading-relaxed", "Led development of scalable web applications using React, Node.js, and PostgreSQL. Architected microservices infrastructure serving 100K+ users. Mentored junior developers and established coding standards." }
                                }
                                div {
                                    class: "text-right md:w-1/3 md:text-right",
                                    p { class: "mb-1 text-sm font-medium", "Mar 2022 - Present" }
                                    p { class: "text-muted-foreground text-sm", "Anthropic" }
                                }
                            }
                        }
                        // Frontend Engineer
                        div {
                            class: "border-border border-b pb-6 last:border-b-0",
                            div {
                                class: "flex flex-col gap-4 md:flex-row md:items-start",
                                div {
                                    class: "md:w-2/3",
                                    div {
                                        class: "mb-2 flex items-center gap-3",
                                        img { class: "h-5 object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/vercel-icon.svg", alt: "Vercel logo" }
                                        h3 { class: "text-xl", "Frontend Engineer" }
                                    }
                                    p { class: "text-muted-foreground mb-3 text-sm", "Full-Time \u{2022} Onsite \u{2022} Austin TX" }
                                    p { class: "text-muted-foreground text-sm leading-relaxed", "Developed full-stack applications using JavaScript, Python, and MySQL. Participated in agile development process and code reviews. Contributed to open-source projects and internal tooling." }
                                }
                                div {
                                    class: "text-right md:w-1/3 md:text-right",
                                    p { class: "mb-1 text-sm font-medium", "Jan 2020 - May 2021" }
                                    p { class: "text-muted-foreground text-sm", "Vercel" }
                                }
                            }
                        }
                        // Junior Web Developer
                        div {
                            class: "border-border border-b pb-6 last:border-b-0",
                            div {
                                class: "flex flex-col gap-4 md:flex-row md:items-start",
                                div {
                                    class: "md:w-2/3",
                                    div {
                                        class: "mb-2 flex items-center gap-3",
                                        img { class: "h-5 object-contain", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/figma-icon.svg", alt: "Figma logo" }
                                        h3 { class: "text-xl", "Junior Web Developer" }
                                    }
                                    p { class: "text-muted-foreground mb-3 text-sm", "Full-Time \u{2022} Onsite \u{2022} Denver CO" }
                                    p { class: "text-muted-foreground text-sm leading-relaxed", "Created responsive websites using HTML, CSS, and JavaScript. Worked with WordPress and PHP for content management. Assisted in debugging and testing web applications." }
                                }
                                div {
                                    class: "text-right md:w-1/3 md:text-right",
                                    p { class: "mb-1 text-sm font-medium", "Aug 2018 - Dec 2019" }
                                    p { class: "text-muted-foreground text-sm", "Figma" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
