use dioxus::prelude::*;

#[component]
pub fn Footer2() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                footer {
                    div {
                        class: "grid grid-cols-2 gap-8 lg:grid-cols-6",
                        div {
                            class: "col-span-2 mb-8 lg:mb-0",
                            div {
                                class: "flex items-center gap-2 lg:justify-start",
                                a {
                                    href: "https://www.shadcnblocks.com",
                                    class: "flex items-center gap-2",
                                    img { class: "h-10", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/block-1.svg", alt: "blocks for shadcn/ui" }
                                    span { class: "text-xl font-semibold", "Shadcnblocks.com" }
                                }
                            }
                            p { class: "mt-4 font-bold", "Components made easy." }
                        }
                        // Product
                        div {
                            h3 { class: "mb-4 font-bold", "Product" }
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li { class: "hover:text-primary font-medium", a { href: "#", "Overview" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Pricing" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Marketplace" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Features" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Integrations" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Pricing" } }
                            }
                        }
                        // Company
                        div {
                            h3 { class: "mb-4 font-bold", "Company" }
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li { class: "hover:text-primary font-medium", a { href: "#", "About" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Team" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Blog" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Careers" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Contact" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Privacy" } }
                            }
                        }
                        // Resources
                        div {
                            h3 { class: "mb-4 font-bold", "Resources" }
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li { class: "hover:text-primary font-medium", a { href: "#", "Help" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Sales" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Advertise" } }
                            }
                        }
                        // Social
                        div {
                            h3 { class: "mb-4 font-bold", "Social" }
                            ul {
                                class: "text-muted-foreground space-y-4",
                                li { class: "hover:text-primary font-medium", a { href: "#", "Twitter" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "Instagram" } }
                                li { class: "hover:text-primary font-medium", a { href: "#", "LinkedIn" } }
                            }
                        }
                    }
                    div {
                        class: "text-muted-foreground mt-24 flex flex-col justify-between gap-4 border-t pt-8 text-sm font-medium md:flex-row md:items-center",
                        p { "\u{00A9} 2024 Shadcnblocks.com. All rights reserved." }
                        ul {
                            class: "flex gap-4",
                            li { class: "hover:text-primary underline", a { href: "#", "Terms and Conditions" } }
                            li { class: "hover:text-primary underline", a { href: "#", "Privacy Policy" } }
                        }
                    }
                }
            }
        }
    }
}
