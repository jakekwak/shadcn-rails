use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn ButtonGroup(
    #[props(default)] orientation: ButtonGroupOrientation,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let orientation_class = match orientation {
        ButtonGroupOrientation::Horizontal => {
            "flex-row [&>*:first-child]:rounded-r-none [&>*:last-child]:rounded-l-none [&>*:not(:first-child):not(:last-child)]:rounded-none [&>*:not(:first-child)]:-ml-px"
        }
        ButtonGroupOrientation::Vertical => {
            "flex-col [&>*:first-child]:rounded-b-none [&>*:last-child]:rounded-t-none [&>*:not(:first-child):not(:last-child)]:rounded-none [&>*:not(:first-child)]:-mt-px"
        }
    };
    rsx! {
        div {
            role: "group",
            class: "inline-flex {orientation_class} {class}",
            {children}
        }
    }
}
