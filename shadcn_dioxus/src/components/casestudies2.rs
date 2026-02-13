use dioxus::prelude::*;
use crate::components::Separator;

#[component]
pub fn Casestudies2() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "flex flex-col gap-6 text-center",
                    p { class: "font-medium", "4500+ Satisfied Customers" }
                    h2 { class: "text-4xl font-medium md:text-5xl", "Real results from real users" }
                }
                div {
                    class: "mt-20",
                    div {
                        class: "grid gap-16 lg:grid-cols-3 xl:gap-24",
                        div {
                            class: "border-border flex flex-col gap-10 sm:flex-row lg:col-span-2 lg:border-r lg:pr-16 xl:pr-24",
                            img {
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                                alt: "placeholder",
                                class: "aspect-[29/35] h-full w-full max-w-60 rounded-2xl object-cover",
                            }
                            div {
                                class: "flex h-full flex-col justify-between gap-10",
                                q { class: "sm:text-xl",
                                    "This productivity tool transformed how we collaborate. Our team\u{2019}s workflow improved dramatically, and we\u{2019}ve cut meeting time by half while increasing output."
                                }
                                div {
                                    class: "flex items-end gap-6",
                                    div {
                                        class: "flex flex-col gap-1",
                                        p { class: "text-primary text-lg font-semibold", "Michael Rivera" }
                                        p { class: "text-muted-foreground", "Product Director" }
                                    }
                                    img {
                                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/vercel-wordmark.svg",
                                        alt: "logo",
                                        class: "h-6 w-auto",
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex gap-10 self-center lg:flex-col",
                            div {
                                class: "flex flex-col gap-2",
                                p { class: "text-primary text-4xl font-medium sm:text-5xl", "98%" }
                                p { class: "text-primary font-semibold", "Customer Satisfaction" }
                                p { class: "text-muted-foreground", "From verified reviews" }
                            }
                            div {
                                class: "flex flex-col gap-2",
                                p { class: "text-primary text-4xl font-medium sm:text-5xl", "3.8x" }
                                p { class: "text-primary font-semibold", "ROI Improvement" }
                                p { class: "text-muted-foreground", "Within first quarter" }
                            }
                        }
                    }
                    Separator { class: "my-20" }
                    div {
                        class: "grid gap-16 lg:grid-cols-3 xl:gap-24",
                        div {
                            class: "border-border flex flex-col gap-10 sm:flex-row lg:col-span-2 lg:border-r lg:pr-16 xl:pr-24",
                            img {
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-2.svg",
                                alt: "placeholder",
                                class: "aspect-[29/35] h-full w-full max-w-60 rounded-2xl object-cover",
                            }
                            div {
                                class: "flex h-full flex-col justify-between gap-10",
                                q { class: "sm:text-xl",
                                    "The interface is intuitive and customizable to our needs. We implemented it across departments with minimal training and saw immediate results."
                                }
                                div {
                                    class: "flex items-end gap-6",
                                    div {
                                        class: "flex flex-col gap-1",
                                        p { class: "text-primary text-lg font-semibold", "Sarah Chen" }
                                        p { class: "text-muted-foreground", "Operations Lead" }
                                    }
                                    img {
                                        src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/figma-wordmark.svg",
                                        alt: "logo",
                                        class: "h-6 w-auto",
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex gap-10 self-center lg:flex-col",
                            div {
                                class: "flex flex-col gap-2",
                                p { class: "text-primary text-4xl font-medium sm:text-5xl", "4.2x" }
                                p { class: "text-primary font-semibold", "Team Efficiency" }
                                p { class: "text-muted-foreground", "Proven productivity gains" }
                            }
                            div {
                                class: "flex flex-col gap-2",
                                p { class: "text-primary text-4xl font-medium sm:text-5xl", "72%" }
                                p { class: "text-primary font-semibold", "Reduced Task Time" }
                                p { class: "text-muted-foreground", "Across all projects" }
                            }
                        }
                    }
                }
            }
        }
    }
}
