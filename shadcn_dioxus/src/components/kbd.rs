use dioxus::prelude::*;

#[component]
pub fn Kbd(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        kbd {
            class: "pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100 {class}",
            {children}
        }
    }
}
