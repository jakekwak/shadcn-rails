use dioxus::prelude::*;

#[component]
pub fn NavigationMenu(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        nav {
            class: "relative z-10 flex max-w-max flex-1 items-center justify-center {class}",
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuList(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        ul {
            class: "group flex flex-1 list-none items-center justify-center space-x-1 {class}",
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuItem(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        li { class: "{class}", {children} }
    }
}

#[component]
pub fn NavigationMenuTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 {class}",
            {children}
            // Chevron down indicator
            span { class: "relative top-[1px] ml-1 h-3 w-3 transition duration-300 group-data-[state=open]:rotate-180",
                "▾"
            }
        }
    }
}

#[component]
pub fn NavigationMenuContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "left-0 top-0 w-full md:absolute md:w-auto {class}",
            {children}
        }
    }
}

#[component]
pub fn NavigationMenuLink(
    #[props(default)] href: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground {class}",
            {children}
        }
    }
}
