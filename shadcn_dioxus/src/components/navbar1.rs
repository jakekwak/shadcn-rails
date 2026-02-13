use dioxus::prelude::*;
use crate::components::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent,
    Button, Sheet, SheetContent, SheetTrigger,
    NavigationMenu, NavigationMenuList, NavigationMenuItem,
    NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink,
};

#[component]
pub fn Navbar1() -> Element {
    rsx! {
        section {
            class: "py-4",
            div {
                class: "container",
                // Desktop Menu
                nav {
                    class: "hidden justify-between lg:flex",
                    div {
                        class: "flex items-center gap-6",
                        // Logo
                        a {
                            class: "flex items-center gap-2",
                            href: "https://www.shadcnblocks.com",
                            img {
                                class: "max-h-8",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/shadcnblockscom-icon.svg",
                                alt: "logo",
                            }
                            span {
                                class: "text-lg font-semibold tracking-tighter",
                                "Shadcnblocks.com"
                            }
                        }
                        div {
                            class: "flex items-center",
                            NavigationMenu {
                                NavigationMenuList {
                                    NavigationMenuItem {
                                        a {
                                            class: "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground",
                                            href: "#",
                                            "Home"
                                        }
                                    }
                                    NavigationMenuItem {
                                        class: "relative group",
                                        NavigationMenuTrigger { "Products" }
                                        NavigationMenuContent {
                                            class: "hidden group-hover:block absolute top-full left-0 z-50 w-80 rounded-md border bg-popover p-4 shadow-md",
                                            ul {
                                                class: "grid gap-3",
                                                li {
                                                    NavigationMenuLink {
                                                        href: "#",
                                                        div { class: "text-sm font-medium leading-none", "Analytics" }
                                                        p { class: "text-sm leading-snug text-muted-foreground line-clamp-2", "Powerful analytics to help you understand your users." }
                                                    }
                                                }
                                                li {
                                                    NavigationMenuLink {
                                                        href: "#",
                                                        div { class: "text-sm font-medium leading-none", "Automation" }
                                                        p { class: "text-sm leading-snug text-muted-foreground line-clamp-2", "Automate your workflow with our powerful tools." }
                                                    }
                                                }
                                                li {
                                                    NavigationMenuLink {
                                                        href: "#",
                                                        div { class: "text-sm font-medium leading-none", "Integrations" }
                                                        p { class: "text-sm leading-snug text-muted-foreground line-clamp-2", "Connect your favorite tools and services." }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    NavigationMenuItem {
                                        class: "relative group",
                                        NavigationMenuTrigger { "Resources" }
                                        NavigationMenuContent {
                                            class: "hidden group-hover:block absolute top-full left-0 z-50 w-80 rounded-md border bg-popover p-4 shadow-md",
                                            ul {
                                                class: "grid gap-3",
                                                li {
                                                    NavigationMenuLink {
                                                        href: "#",
                                                        div { class: "text-sm font-medium leading-none", "Blog" }
                                                        p { class: "text-sm leading-snug text-muted-foreground line-clamp-2", "Read our latest articles and updates." }
                                                    }
                                                }
                                                li {
                                                    NavigationMenuLink {
                                                        href: "#",
                                                        div { class: "text-sm font-medium leading-none", "Documentation" }
                                                        p { class: "text-sm leading-snug text-muted-foreground line-clamp-2", "Learn how to use our platform effectively." }
                                                    }
                                                }
                                                li {
                                                    NavigationMenuLink {
                                                        href: "#",
                                                        div { class: "text-sm font-medium leading-none", "Help Center" }
                                                        p { class: "text-sm leading-snug text-muted-foreground line-clamp-2", "Get support and find answers to your questions." }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    NavigationMenuItem {
                                        a {
                                            class: "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground",
                                            href: "#",
                                            "Pricing"
                                        }
                                    }
                                    NavigationMenuItem {
                                        a {
                                            class: "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground",
                                            href: "#",
                                            "Blog"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex gap-2",
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-9 px-3 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                            href: "https://example.com",
                            "Log in"
                        }
                        a {
                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-9 px-3 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                            href: "https://example.com",
                            "Sign up"
                        }
                    }
                }

                // Mobile Menu
                div {
                    class: "block lg:hidden",
                    div {
                        class: "flex items-center justify-between",
                        // Logo
                        a {
                            class: "flex items-center gap-2",
                            href: "https://www.shadcnblocks.com",
                            img {
                                class: "max-h-8",
                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/shadcnblockscom-icon.svg",
                                alt: "logo",
                            }
                        }
                        Sheet {
                            SheetTrigger {
                                Button {
                                    variant: "outline",
                                    size: "icon",
                                    svg {
                                        class: "size-4",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        line { x1: "4", x2: "20", y1: "12", y2: "12" }
                                        line { x1: "4", x2: "20", y1: "6", y2: "6" }
                                        line { x1: "4", x2: "20", y1: "18", y2: "18" }
                                    }
                                }
                            }
                            SheetContent {
                                class: "overflow-y-auto",
                                div {
                                    class: "flex flex-col space-y-2 text-center sm:text-left",
                                    h2 {
                                        class: "text-lg font-semibold text-foreground",
                                        a {
                                            class: "flex items-center gap-2",
                                            href: "https://www.shadcnblocks.com",
                                            img {
                                                class: "max-h-8",
                                                src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/shadcnblockscom-icon.svg",
                                                alt: "logo",
                                            }
                                            span {
                                                class: "text-lg font-semibold tracking-tighter",
                                                "Shadcnblocks.com"
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col gap-6 p-4",
                                    a {
                                        class: "font-semibold",
                                        href: "#",
                                        "Home"
                                    }
                                    Accordion {
                                        class: "flex w-full flex-col gap-4",
                                        accordion_type: "single",
                                        AccordionItem {
                                            value: "products",
                                            AccordionTrigger {
                                                class: "mb-4 py-0 font-semibold hover:no-underline",
                                                "Products"
                                            }
                                            AccordionContent {
                                                div {
                                                    class: "flex flex-col gap-4",
                                                    a {
                                                        class: "text-muted-foreground",
                                                        href: "#",
                                                        "Analytics"
                                                    }
                                                    a {
                                                        class: "text-muted-foreground",
                                                        href: "#",
                                                        "Automation"
                                                    }
                                                    a {
                                                        class: "text-muted-foreground",
                                                        href: "#",
                                                        "Integrations"
                                                    }
                                                }
                                            }
                                        }
                                        AccordionItem {
                                            value: "resources",
                                            AccordionTrigger {
                                                class: "mb-4 py-0 font-semibold hover:no-underline",
                                                "Resources"
                                            }
                                            AccordionContent {
                                                div {
                                                    class: "flex flex-col gap-4",
                                                    a {
                                                        class: "text-muted-foreground",
                                                        href: "#",
                                                        "Blog"
                                                    }
                                                    a {
                                                        class: "text-muted-foreground",
                                                        href: "#",
                                                        "Documentation"
                                                    }
                                                    a {
                                                        class: "text-muted-foreground",
                                                        href: "#",
                                                        "Help Center"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    a {
                                        class: "font-semibold",
                                        href: "#",
                                        "Pricing"
                                    }
                                    a {
                                        class: "font-semibold",
                                        href: "#",
                                        "Blog"
                                    }
                                    div {
                                        class: "flex flex-col gap-3",
                                        a {
                                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                                            href: "https://example.com",
                                            "Log in"
                                        }
                                        a {
                                            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                                            href: "https://example.com",
                                            "Sign up"
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
}
