use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn ResizablePanelGroup(
    #[props(default)] direction: ResizableDirection,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let direction_class = match direction {
        ResizableDirection::Horizontal => "flex-row",
        ResizableDirection::Vertical => "flex-col",
    };
    let data_direction = match direction {
        ResizableDirection::Horizontal => "horizontal",
        ResizableDirection::Vertical => "vertical",
    };
    rsx! {
        div {
            "data-panel-group": "",
            "data-direction": "{data_direction}",
            class: "flex h-full w-full {direction_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn ResizablePanel(
    #[props(default = 50.0)] default_size: f64,
    #[props(default)] min_size: Option<f64>,
    #[props(default)] max_size: Option<f64>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let style = format!("flex-basis: {}%; flex-grow: 0; flex-shrink: 0;", default_size);
    let min_w = min_size.map(|s| format!("min-width: {}%;", s)).unwrap_or_default();
    let max_w = max_size.map(|s| format!("max-width: {}%;", s)).unwrap_or_default();
    let full_style = format!("{style} {min_w} {max_w}");

    rsx! {
        div {
            "data-panel": "",
            class: "overflow-hidden {class}",
            style: "{full_style}",
            {children}
        }
    }
}

#[component]
pub fn ResizableHandle(
    #[props(default)] with_handle: bool,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div {
            "data-panel-resize-handle": "",
            class: "relative flex w-px items-center justify-center bg-border after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1 data-[direction=vertical]:h-px data-[direction=vertical]:w-full data-[direction=vertical]:after:left-0 data-[direction=vertical]:after:h-1 data-[direction=vertical]:after:w-full data-[direction=vertical]:after:-translate-y-1/2 data-[direction=vertical]:after:translate-x-0 [&[data-direction=vertical]>div]:rotate-90 {class}",
            if with_handle {
                div {
                    class: "z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-border",
                    // Grip dots
                    svg {
                        class: "h-2.5 w-2.5",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 6 16",
                        fill: "currentColor",
                        circle { cx: "1", cy: "4", r: "0.7" }
                        circle { cx: "1", cy: "8", r: "0.7" }
                        circle { cx: "1", cy: "12", r: "0.7" }
                        circle { cx: "5", cy: "4", r: "0.7" }
                        circle { cx: "5", cy: "8", r: "0.7" }
                        circle { cx: "5", cy: "12", r: "0.7" }
                    }
                }
            }
        }
    }
}
