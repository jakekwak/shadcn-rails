use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let variant_class = match variant {
        AlertVariant::Default => "bg-background text-foreground [&>svg]:text-foreground",
        AlertVariant::Destructive => {
            "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
        }
    };
    rsx! {
        div {
            role: "alert",
            class: "relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg~*]:pl-7 {variant_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn AlertTitle(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        h5 {
            class: "mb-1 font-medium leading-none tracking-tight {class}",
            {children}
        }
    }
}

#[component]
pub fn AlertDescription(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "text-sm [&_p]:leading-relaxed {class}",
            {children}
        }
    }
}
