use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Spinner(
    #[props(default)] size: SpinnerSize,
    #[props(default)] class: String,
) -> Element {
    let size_class = match size {
        SpinnerSize::Sm => "h-4 w-4",
        SpinnerSize::Md => "h-6 w-6",
        SpinnerSize::Lg => "h-8 w-8",
    };
    rsx! {
        svg {
            class: "animate-spin {size_class} {class}",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            circle {
                class: "opacity-25",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                class: "opacity-75",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
            }
        }
    }
}
