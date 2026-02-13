use dioxus::prelude::*;

#[component]
pub fn Contact7() -> Element {
    rsx! {
        section {
            class: "bg-background py-32",
            div {
                class: "container",
                div {
                    class: "mb-14",
                    h1 {class: "mb-3 mt-2 text-balance text-3xl font-semibold md:text-4xl",
                        "Contact Us"}
                    p {class: "text-muted-foreground max-w-xl text-lg",
                        "Contact the support team at Shadcnblocks."}
                }
                div {
                    class: "grid gap-6 md:grid-cols-2",
                    div {
                        class: "bg-muted rounded-lg p-6",
                        span {
                            class: "bg-accent mb-3 flex size-12 flex-col items-center justify-center rounded-full",
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", rect {width: "20", height: "16", x: "2", y: "4", rx: "2"} path {d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"} }
                        }
                        p {class: "mb-2 text-lg font-semibold",
                            "Email"}
                        p {class: "text-muted-foreground mb-3",
                            "We respond to all emails within 24 hours."}
                        a {class: "font-semibold hover:underline", href: "mailto:",
                            "example@shadcnblocks.com"}
                    }
                    div {
                        class: "bg-muted rounded-lg p-6",
                        span {
                            class: "bg-accent mb-3 flex size-12 flex-col items-center justify-center rounded-full",
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"} circle {cx: "12", cy: "10", r: "3"} }
                        }
                        p {class: "mb-2 text-lg font-semibold",
                            "Office"}
                        p {class: "text-muted-foreground mb-3",
                            "Drop by our office for a chat."}
                        a {class: "font-semibold hover:underline", href: "https://example.com",
                            "1 Eagle St, Brisbane, QLD, 4000"}
                    }
                    div {
                        class: "bg-muted rounded-lg p-6",
                        span {
                            class: "bg-accent mb-3 flex size-12 flex-col items-center justify-center rounded-full",
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"} }
                        }
                        p {class: "mb-2 text-lg font-semibold",
                            "Phone"}
                        p {class: "text-muted-foreground mb-3",
                            "We're available Mon-Fri, 9am-5pm."}
                        a {class: "font-semibold hover:underline", href: "tel:",
                            "+123 456 7890"}
                    }
                    div {
                        class: "bg-muted rounded-lg p-6",
                        span {
                            class: "bg-accent mb-3 flex size-12 flex-col items-center justify-center rounded-full",
                            svg { class: "size-4", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path {d: "M7.9 20A9 9 0 1 0 4 16.1L2 22Z"} }
                        }
                        p {class: "mb-2 text-lg font-semibold",
                            "Live Chat"}
                        p {class: "text-muted-foreground mb-3",
                            "Get instant help from our support team."}
                        a {class: "font-semibold hover:underline", href: "https://example.com",
                            "Start Chat"}
                    }
                }
            }
        }
    }
}
