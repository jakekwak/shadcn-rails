use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn Carousel(
    #[props(default)] orientation: CarouselOrientation,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let orientation_class = match orientation {
        CarouselOrientation::Horizontal => "",
        CarouselOrientation::Vertical => "flex-col",
    };

    rsx! {
        div {
            role: "region",
            "aria-roledescription": "carousel",
            "aria-label": "Carousel",
            class: "relative {orientation_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn CarouselContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "overflow-hidden",
            div {
                class: "flex -ml-4 {class}",
                {children}
            }
        }
    }
}

#[component]
pub fn CarouselItem(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            role: "group",
            "aria-roledescription": "slide",
            class: "min-w-0 shrink-0 grow-0 basis-full pl-4 {class}",
            {children}
        }
    }
}

#[component]
pub fn CarouselPrevious(
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "absolute left-[-12px] top-1/2 -translate-y-1/2 inline-flex items-center justify-center h-8 w-8 rounded-full border bg-background shadow-sm hover:bg-accent hover:text-accent-foreground disabled:opacity-50 {class}",
            onclick: move |e| onclick.call(e),
            "aria-label": "Previous slide",
            "←"
        }
    }
}

#[component]
pub fn CarouselNext(
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "absolute right-[-12px] top-1/2 -translate-y-1/2 inline-flex items-center justify-center h-8 w-8 rounded-full border bg-background shadow-sm hover:bg-accent hover:text-accent-foreground disabled:opacity-50 {class}",
            onclick: move |e| onclick.call(e),
            "aria-label": "Next slide",
            "→"
        }
    }
}
