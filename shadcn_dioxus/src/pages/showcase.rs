use dioxus::prelude::*;
use crate::components::{
    alert::{Alert, AlertDescription, AlertTitle, AlertVariant},
    breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator},
    button_group::{ButtonGroup, ButtonGroupOrientation},
    carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious},
    drawer::{Drawer, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle},
    input_otp::InputOTP,
    kbd::Kbd,
    navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList},
    pagination::{Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious},
    resizable::{ResizableHandle, ResizablePanel, ResizablePanelGroup},
    spinner::{Spinner, SpinnerSize},
    table::{Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow},
    typography::{Blockquote, H1, H2, H3, H4, InlineCode, Large, Lead, Muted, P, Small},
};

/// Section wrapper for the showcase
#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        section { class: "space-y-4",
            h2 { class: "scroll-m-20 border-b pb-2 text-2xl font-semibold tracking-tight",
                "{title}"
            }
            div { class: "space-y-4",
                {children}
            }
        }
    }
}

#[component]
pub fn ShowcasePage() -> Element {
    let mut drawer_open = use_signal(|| false);
    let otp_value = use_signal(|| String::new());

    rsx! {
        div { class: "container mx-auto max-w-4xl py-10 px-4 space-y-12",
            // Page Title
            H1 { "shadcn/ui Dioxus Components" }
            Lead { "13 missing components implemented in Dioxus RSX + Tailwind CSS" }

            // ── Alert ──
            Section { title: "Alert",
                Alert {
                    AlertTitle { "Heads up!" }
                    AlertDescription { "You can add components to your app using the CLI." }
                }
                Alert { variant: AlertVariant::Destructive,
                    AlertTitle { "Error" }
                    AlertDescription { "Your session has expired. Please log in again." }
                }
            }

            // ── Breadcrumb ──
            Section { title: "Breadcrumb",
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbLink { href: "/", "Home" }
                        }
                        BreadcrumbSeparator {}
                        BreadcrumbItem {
                            BreadcrumbLink { href: "/components", "Components" }
                        }
                        BreadcrumbSeparator {}
                        BreadcrumbItem {
                            BreadcrumbPage { "Breadcrumb" }
                        }
                    }
                }
            }

            // ── ButtonGroup ──
            Section { title: "ButtonGroup",
                ButtonGroup {
                    button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                        "Left"
                    }
                    button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                        "Center"
                    }
                    button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                        "Right"
                    }
                }
                ButtonGroup { orientation: ButtonGroupOrientation::Vertical,
                    button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                        "Top"
                    }
                    button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                        "Bottom"
                    }
                }
            }

            // ── Kbd ──
            Section { title: "Kbd",
                div { class: "flex items-center gap-2",
                    Kbd { "⌘" }
                    Kbd { "K" }
                    span { class: "text-sm text-muted-foreground", "to open command palette" }
                }
                div { class: "flex items-center gap-1",
                    Kbd { "Ctrl" }
                    span { class: "text-muted-foreground", "+" }
                    Kbd { "C" }
                }
            }

            // ── Spinner ──
            Section { title: "Spinner",
                div { class: "flex items-center gap-4",
                    Spinner { size: SpinnerSize::Sm }
                    Spinner {}
                    Spinner { size: SpinnerSize::Lg }
                }
            }

            // ── Typography ──
            Section { title: "Typography",
                H1 { "Heading 1" }
                H2 { "Heading 2" }
                H3 { "Heading 3" }
                H4 { "Heading 4" }
                P { "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the taxes." }
                Lead { "A lead paragraph stands out from regular text." }
                Large { "Large text for emphasis." }
                Small { "Small text for details." }
                Muted { "Muted text for secondary info." }
                Blockquote { "\"After all,\" he said, \"everyone enjoys a good quote.\"" }
                P {
                    "Use "
                    InlineCode { "code" }
                    " for inline code snippets."
                }
            }

            // ── Table ──
            Section { title: "Table",
                Table {
                    TableCaption { "A list of your recent invoices." }
                    TableHeader {
                        TableRow {
                            TableHead { "Invoice" }
                            TableHead { "Status" }
                            TableHead { "Method" }
                            TableHead { class: "text-right", "Amount" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { class: "font-medium", "INV001" }
                            TableCell { "Paid" }
                            TableCell { "Credit Card" }
                            TableCell { class: "text-right", "$250.00" }
                        }
                        TableRow {
                            TableCell { class: "font-medium", "INV002" }
                            TableCell { "Pending" }
                            TableCell { "PayPal" }
                            TableCell { class: "text-right", "$150.00" }
                        }
                        TableRow {
                            TableCell { class: "font-medium", "INV003" }
                            TableCell { "Unpaid" }
                            TableCell { "Bank Transfer" }
                            TableCell { class: "text-right", "$350.00" }
                        }
                    }
                }
            }

            // ── Pagination ──
            Section { title: "Pagination",
                Pagination {
                    PaginationContent {
                        PaginationItem { PaginationPrevious { href: "#" } }
                        PaginationItem { PaginationLink { href: "#", "1" } }
                        PaginationItem { PaginationLink { href: "#", is_active: true, "2" } }
                        PaginationItem { PaginationLink { href: "#", "3" } }
                        PaginationItem { PaginationEllipsis {} }
                        PaginationItem { PaginationNext { href: "#" } }
                    }
                }
            }

            // ── Drawer ──
            Section { title: "Drawer",
                button {
                    class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                    onclick: move |_| drawer_open.set(true),
                    "Open Drawer"
                }
                Drawer { open: drawer_open,
                    DrawerHeader {
                        DrawerTitle { "Edit profile" }
                        DrawerDescription { "Make changes to your profile here." }
                    }
                    div { class: "p-4",
                        p { class: "text-sm text-muted-foreground", "Drawer content goes here." }
                    }
                    DrawerFooter {
                        button {
                            class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary text-primary-foreground shadow hover:bg-primary/90",
                            onclick: move |_| drawer_open.set(false),
                            "Save changes"
                        }
                        button {
                            class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
                            onclick: move |_| drawer_open.set(false),
                            "Cancel"
                        }
                    }
                }
            }

            // ── NavigationMenu ──
            Section { title: "NavigationMenu",
                NavigationMenu {
                    NavigationMenuList {
                        NavigationMenuItem {
                            NavigationMenuLink { href: "#", "Getting Started" }
                        }
                        NavigationMenuItem {
                            NavigationMenuLink { href: "#", "Components" }
                        }
                        NavigationMenuItem {
                            NavigationMenuLink { href: "#", "Documentation" }
                        }
                    }
                }
            }

            // ── InputOTP ──
            Section { title: "InputOTP",
                div { class: "flex items-center gap-4",
                    InputOTP { length: 6, value: otp_value }
                    p { class: "text-sm text-muted-foreground",
                        "Value: {otp_value}"
                    }
                }
            }

            // ── Carousel ──
            Section { title: "Carousel",
                div { class: "w-full max-w-xs mx-auto",
                    Carousel {
                        CarouselContent {
                            for i in 1..=5 {
                                CarouselItem {
                                    div { class: "p-1",
                                        div { class: "flex aspect-square items-center justify-center rounded-md border bg-card p-6",
                                            span { class: "text-3xl font-semibold", "{i}" }
                                        }
                                    }
                                }
                            }
                        }
                        CarouselPrevious {}
                        CarouselNext {}
                    }
                }
            }

            // ── Resizable ──
            Section { title: "Resizable",
                div { class: "h-[200px] rounded-lg border",
                    ResizablePanelGroup {
                        ResizablePanel { default_size: 25.0,
                            div { class: "flex h-full items-center justify-center p-6",
                                span { class: "font-semibold", "Sidebar" }
                            }
                        }
                        ResizableHandle { with_handle: true }
                        ResizablePanel { default_size: 75.0,
                            div { class: "flex h-full items-center justify-center p-6",
                                span { class: "font-semibold", "Content" }
                            }
                        }
                    }
                }
            }

            // Footer
            div { class: "border-t pt-6 text-center text-sm text-muted-foreground",
                "Built with Dioxus + Tailwind CSS • shadcn/ui component port"
            }
        }
    }
}
