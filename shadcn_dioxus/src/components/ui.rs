use dioxus::prelude::*;

// ── Accordion (interactive) ──────────────────────────────────────────

#[derive(Clone)]
struct AccordionContext {
    open_value: Signal<String>,
}

#[derive(Clone)]
struct AccordionItemContext {
    value: String,
}

#[component]
pub fn Accordion(
    #[props(default)] class: String,
    #[props(default)] accordion_type: String,
    #[props(default)] default_value: String,
    children: Element,
) -> Element {
    let open_value = use_signal(|| default_value.clone());
    use_context_provider(|| AccordionContext { open_value });
    rsx! {
        div { class: "divide-y {class}", {children} }
    }
}

#[component]
pub fn AccordionItem(
    #[props(default)] class: String,
    #[props(default)] value: String,
    children: Element,
) -> Element {
    use_context_provider(|| AccordionItemContext { value: value.clone() });
    rsx! {
        div { class: "border-b {class}", "data-value": "{value}", {children} }
    }
}

#[component]
pub fn AccordionTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut ctx = use_context::<AccordionContext>();
    let item_ctx = use_context::<AccordionItemContext>();
    let is_open = *ctx.open_value.read() == item_ctx.value;
    let chevron_class = if is_open { "rotate-180" } else { "" };
    let item_value = item_ctx.value.clone();
    rsx! {
        button {
            class: "flex w-full items-center justify-between py-4 text-left font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180 {class}",
            "data-state": if is_open { "open" } else { "closed" },
            onclick: move |_| {
                let current = ctx.open_value.read().clone();
                if current == item_value {
                    ctx.open_value.set(String::new());
                } else {
                    ctx.open_value.set(item_value.clone());
                }
            },
            {children}
            svg {
                class: "h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200 {chevron_class}",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<AccordionContext>();
    let item_ctx = use_context::<AccordionItemContext>();
    let is_open = *ctx.open_value.read() == item_ctx.value;
    if is_open {
        rsx! {
            div {
                class: "overflow-hidden text-sm",
                "data-state": "open",
                div { class: "pb-4 pt-0 {class}", {children} }
            }
        }
    } else {
        rsx! {}
    }
}

#[component]
pub fn Avatar(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        span { class: "relative flex shrink-0 overflow-hidden rounded-full {class}", {children} }
    }
}

#[component]
pub fn AvatarImage(
    #[props(default)] class: String,
    #[props(default)] src: String,
    #[props(default)] alt: String,
) -> Element {
    rsx! {
        img { class: "aspect-square h-full w-full {class}", src: "{src}", alt: "{alt}" }
    }
}

#[component]
pub fn AvatarFallback(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        span { class: "flex h-full w-full items-center justify-center rounded-full bg-muted {class}", {children} }
    }
}

#[component]
pub fn Badge(
    #[props(default)] class: String,
    #[props(default)] variant: String,
    children: Element,
) -> Element {
    let variant_class = match variant.as_str() {
        "secondary" => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
        "outline" => "text-foreground border",
        "destructive" => "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80",
        _ => "border-transparent bg-primary text-primary-foreground hover:bg-primary/80",
    };
    rsx! {
        div { class: "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors {variant_class} {class}", {children} }
    }
}

#[component]
pub fn Button(
    #[props(default)] class: String,
    #[props(default)] variant: String,
    #[props(default)] size: String,
    children: Element,
) -> Element {
    let variant_class = match variant.as_str() {
        "outline" => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        "ghost" => "hover:bg-accent hover:text-accent-foreground",
        "link" => "text-primary underline-offset-4 hover:underline",
        "destructive" => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        "secondary" => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        _ => "bg-primary text-primary-foreground hover:bg-primary/90",
    };
    let size_class = match size.as_str() {
        "sm" => "h-8 rounded-md px-3 text-xs",
        "lg" => "h-10 rounded-md px-8",
        "icon" => "h-9 w-9",
        _ => "h-9 px-4 py-2",
    };
    rsx! {
        button { class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 {variant_class} {size_class} {class}", {children} }
    }
}

#[component]
pub fn Input(
    #[props(default)] class: String,
    #[props(default)] placeholder: String,
    #[props(default)] r#type: String,
) -> Element {
    let input_type = if r#type.is_empty() { "text" } else { &r#type };
    rsx! {
        input {
            class: "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 {class}",
            r#type: "{input_type}",
            placeholder: "{placeholder}",
        }
    }
}

#[component]
pub fn Label(
    #[props(default)] class: String,
    #[props(default)] r#for: String,
    children: Element,
) -> Element {
    rsx! {
        label { class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {class}", r#for: "{r#for}", {children} }
    }
}

#[component]
pub fn Separator(
    #[props(default)] class: String,
    #[props(default)] orientation: String,
) -> Element {
    let orientation_class = if orientation == "vertical" {
        "h-full w-[1px]"
    } else {
        "h-[1px] w-full"
    };
    rsx! {
        div { role: "separator", class: "shrink-0 bg-border {orientation_class} {class}" }
    }
}

#[component]
pub fn Sheet(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "{class}", {children} }
    }
}

#[component]
pub fn SheetTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "{class}", {children} }
    }
}

#[component]
pub fn SheetContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "fixed inset-y-0 right-0 z-50 flex h-full w-3/4 flex-col border-l bg-background p-6 shadow-lg sm:max-w-sm {class}", {children} }
    }
}

#[component]
pub fn Switch(
    #[props(default)] class: String,
    #[props(default)] checked: bool,
) -> Element {
    let bg = if checked { "bg-primary" } else { "bg-input" };
    rsx! {
        button {
            role: "switch",
            class: "peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {bg} {class}",
            span { class: "pointer-events-none block h-4 w-4 rounded-full bg-background shadow-lg ring-0 transition-transform" }
        }
    }
}

#[component]
pub fn Tabs(
    #[props(default)] class: String,
    #[props(default)] default_value: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "{class}", "data-default-value": "{default_value}", {children} }
    }
}

#[component]
pub fn TabsList(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground {class}", {children} }
    }
}

#[component]
pub fn TabsTrigger(
    #[props(default)] class: String,
    #[props(default)] value: String,
    children: Element,
) -> Element {
    rsx! {
        button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {class}", "data-value": "{value}", {children} }
    }
}

#[component]
pub fn TabsContent(
    #[props(default)] class: String,
    #[props(default)] value: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {class}", "data-value": "{value}", {children} }
    }
}

#[component]
pub fn Tooltip(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "relative inline-block {class}", {children} }
    }
}

#[component]
pub fn TooltipTrigger(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "{class}", {children} }
    }
}

#[component]
pub fn TooltipContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground animate-in fade-in-0 zoom-in-95 {class}", {children} }
    }
}
