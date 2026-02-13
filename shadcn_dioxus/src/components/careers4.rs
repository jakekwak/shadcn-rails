use dioxus::prelude::*;
use crate::components::Button;

#[component]
pub fn Careers4() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mx-auto max-w-xl",
                    div {
                        class: "text-center lg:text-left",
                        h1 {
                            class: "text-left text-3xl font-medium md:text-4xl",
                            "Job Openings"
                        }
                    }
                    div {
                        class: "mx-auto mt-6 flex flex-col gap-16 md:mt-14",
                        // Engineering
                        div {
                            class: "grid",
                            h2 { class: "border-b pb-4 text-xl font-bold", "Engineering" }
                            div {
                                class: "flex items-center justify-between border-b py-4",
                                a { class: "font-semibold hover:underline", href: "#", "Senior Frontend Developer" }
                                Button { class: "pointer-events-none rounded-full", variant: "outline", size: "sm", "Remote" }
                            }
                            div {
                                class: "flex items-center justify-between border-b py-4",
                                a { class: "font-semibold hover:underline", href: "#", "UI/UX Designer" }
                                Button { class: "pointer-events-none rounded-full", variant: "outline", size: "sm", "San Francisco" }
                            }
                            div {
                                class: "flex items-center justify-between border-b py-4",
                                a { class: "font-semibold hover:underline", href: "#", "React Developer" }
                                Button { class: "pointer-events-none rounded-full", variant: "outline", size: "sm", "Remote" }
                            }
                            div {
                                class: "flex items-center justify-between border-b py-4",
                                a { class: "font-semibold hover:underline", href: "#", "Technical Lead" }
                                Button { class: "pointer-events-none rounded-full", variant: "outline", size: "sm", "London" }
                            }
                        }
                        // Design
                        div {
                            class: "grid",
                            h2 { class: "border-b pb-4 text-xl font-bold", "Design" }
                            div {
                                class: "flex items-center justify-between border-b py-4",
                                a { class: "font-semibold hover:underline", href: "#", "Product Designer" }
                                Button { class: "pointer-events-none rounded-full", variant: "outline", size: "sm", "Remote" }
                            }
                            div {
                                class: "flex items-center justify-between border-b py-4",
                                a { class: "font-semibold hover:underline", href: "#", "Visual Designer" }
                                Button { class: "pointer-events-none rounded-full", variant: "outline", size: "sm", "Berlin" }
                            }
                        }
                    }
                }
            }
        }
    }
}
