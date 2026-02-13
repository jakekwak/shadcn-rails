use dioxus::prelude::*;

#[component]
pub fn Breadcrumb(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        nav {
            "aria-label": "breadcrumb",
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbList(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        ol {
            class: "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 {class}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbItem(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        li {
            class: "inline-flex items-center gap-1.5 {class}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbLink(
    #[props(default)] href: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "transition-colors hover:text-foreground {class}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbPage(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        span {
            role: "link",
            "aria-disabled": "true",
            "aria-current": "page",
            class: "font-normal text-foreground {class}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(#[props(default)] class: String) -> Element {
    rsx! {
        li {
            role: "presentation",
            "aria-hidden": "true",
            class: "[&>svg]:h-3.5 [&>svg]:w-3.5 {class}",
            // Chevron right as text fallback
            span { "/" }
        }
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[props(default)] class: String) -> Element {
    rsx! {
        span {
            role: "presentation",
            "aria-hidden": "true",
            class: "flex h-9 w-9 items-center justify-center {class}",
            "…"
        }
    }
}
