use dioxus::prelude::*;

#[component]
pub fn Pagination(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        nav {
            role: "navigation",
            "aria-label": "pagination",
            class: "mx-auto flex w-full justify-center {class}",
            {children}
        }
    }
}

#[component]
pub fn PaginationContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        ul {
            class: "flex flex-row items-center gap-1 {class}",
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        li {
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn PaginationLink(
    #[props(default)] href: String,
    #[props(default)] is_active: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let active_class = if is_active {
        "border border-input bg-background shadow-sm"
    } else {
        "hover:bg-accent hover:text-accent-foreground"
    };
    rsx! {
        a {
            href: "{href}",
            "aria-current": if is_active { "page" },
            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 h-9 w-9 {active_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn PaginationPrevious(
    #[props(default)] href: String,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        PaginationLink { href: "{href}", class: "gap-1 pl-2.5 w-auto {class}",
            span { class: "text-sm", "←" }
            span { "Previous" }
        }
    }
}

#[component]
pub fn PaginationNext(
    #[props(default)] href: String,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        PaginationLink { href: "{href}", class: "gap-1 pr-2.5 w-auto {class}",
            span { "Next" }
            span { class: "text-sm", "→" }
        }
    }
}

#[component]
pub fn PaginationEllipsis(#[props(default)] class: String) -> Element {
    rsx! {
        span {
            "aria-hidden": "true",
            class: "flex h-9 w-9 items-center justify-center {class}",
            "…"
        }
    }
}
