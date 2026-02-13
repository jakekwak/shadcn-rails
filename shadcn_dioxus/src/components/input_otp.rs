use dioxus::prelude::*;

#[component]
pub fn InputOTP(
    length: usize,
    value: Signal<String>,
    #[props(default)] class: String,
) -> Element {
    let current_value = value();

    rsx! {
        div {
            class: "flex items-center gap-2 has-[:disabled]:opacity-50 {class}",
            for i in 0..length {
                InputOTPSlot {
                    index: i,
                    char_value: current_value.chars().nth(i),
                    is_active: current_value.len() == i,
                }
            }
            // Hidden input for actual value
            input {
                r#type: "text",
                inputmode: "numeric",
                autocomplete: "one-time-code",
                pattern: "[0-9]*",
                maxlength: "{length}",
                value: "{current_value}",
                class: "sr-only",
                oninput: move |e| value.set(e.value()),
            }
        }
    }
}

#[component]
fn InputOTPSlot(
    index: usize,
    #[props(default)] char_value: Option<char>,
    #[props(default)] is_active: bool,
) -> Element {
    let active_class = if is_active {
        "z-10 ring-1 ring-ring"
    } else {
        ""
    };
    let border_class = if index > 0 {
        "border-l-0"
    } else {
        ""
    };

    rsx! {
        div {
            class: "relative flex h-9 w-9 items-center justify-center border-y border-r border-input text-sm shadow-sm transition-all first:rounded-l-md first:border-l last:rounded-r-md {active_class} {border_class}",
            if let Some(c) = char_value {
                span { "{c}" }
            } else if is_active {
                div { class: "absolute inset-0 flex items-center justify-center",
                    div { class: "h-4 w-px animate-pulse bg-foreground duration-1000" }
                }
            }
        }
    }
}

#[component]
pub fn InputOTPGroup(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center {class}",
            {children}
        }
    }
}

#[component]
pub fn InputOTPSeparator(#[props(default)] class: String) -> Element {
    rsx! {
        div {
            role: "separator",
            class: "flex items-center {class}",
            span { class: "text-muted-foreground", "−" }
        }
    }
}
