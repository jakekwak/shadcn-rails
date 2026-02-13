use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum DrawerSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[component]
pub fn Drawer(
    open: Signal<bool>,
    #[props(default)] side: DrawerSide,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let is_open = open();

    if !is_open {
        return rsx! {};
    }

    let (translate_class, position_class) = match side {
        DrawerSide::Top => (
            "inset-x-0 top-0 border-b",
            "translate-y-0",
        ),
        DrawerSide::Bottom => (
            "inset-x-0 bottom-0 border-t",
            "translate-y-0",
        ),
        DrawerSide::Left => (
            "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
            "translate-x-0",
        ),
        DrawerSide::Right => (
            "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
            "translate-x-0",
        ),
    };

    rsx! {
        // Overlay
        div {
            class: "fixed inset-0 z-50 bg-black/80",
            onclick: move |_| open.set(false),
        }
        // Content
        div {
            class: "fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out {position_class} {translate_class} {class}",
            {children}
            // Handle for bottom drawer
            if matches!(side, DrawerSide::Bottom) {
                div { class: "mx-auto mt-4 h-2 w-[100px] rounded-full bg-muted" }
            }
        }
    }
}

#[component]
pub fn DrawerHeader(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "grid gap-1.5 p-4 text-center sm:text-left {class}",
            {children}
        }
    }
}

#[component]
pub fn DrawerFooter(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "mt-auto flex flex-col gap-2 p-4 {class}",
            {children}
        }
    }
}

#[component]
pub fn DrawerTitle(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        h3 {
            class: "text-lg font-semibold leading-none tracking-tight {class}",
            {children}
        }
    }
}

#[component]
pub fn DrawerDescription(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground {class}",
            {children}
        }
    }
}
