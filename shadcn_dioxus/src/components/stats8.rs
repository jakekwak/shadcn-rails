use dioxus::prelude::*;

#[component]
pub fn Stats8() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "flex flex-col gap-4",
                    h2 { class: "text-2xl font-bold md:text-4xl", "Platform performance insights" }
                    p { "Ensuring stability and scalability for all users" }
                    a {
                        class: "flex items-center gap-1 font-bold hover:underline",
                        href: "https://www.shadcnblocks.com",
                        "Read the full impact report"
                        svg { class: "h-auto w-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } path { d: "m12 5 7 7-7 7" } }
                    }
                }
                div {
                    class: "mt-14 grid gap-x-5 gap-y-8 md:grid-cols-2 lg:grid-cols-4",
                    div {
                        class: "flex flex-col gap-5",
                        div { class: "text-6xl font-bold", "250%+" }
                        p { "average growth in user engagement" }
                    }
                    div {
                        class: "flex flex-col gap-5",
                        div { class: "text-6xl font-bold", "$2.5m" }
                        p { "annual savings per enterprise partner" }
                    }
                    div {
                        class: "flex flex-col gap-5",
                        div { class: "text-6xl font-bold", "200+" }
                        p { "integrations with top industry platforms" }
                    }
                    div {
                        class: "flex flex-col gap-5",
                        div { class: "text-6xl font-bold", "99.9%" }
                        p { "customer satisfaction over the last year" }
                    }
                }
            }
        }
    }
}
