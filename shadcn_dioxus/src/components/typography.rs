use dioxus::prelude::*;

#[component]
pub fn H1(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        h1 {
            class: "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl {class}",
            {children}
        }
    }
}

#[component]
pub fn H2(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        h2 {
            class: "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0 {class}",
            {children}
        }
    }
}

#[component]
pub fn H3(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        h3 {
            class: "scroll-m-20 text-2xl font-semibold tracking-tight {class}",
            {children}
        }
    }
}

#[component]
pub fn H4(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        h4 {
            class: "scroll-m-20 text-xl font-semibold tracking-tight {class}",
            {children}
        }
    }
}

#[component]
pub fn P(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        p {
            class: "leading-7 [&:not(:first-child)]:mt-6 {class}",
            {children}
        }
    }
}

#[component]
pub fn Blockquote(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        blockquote {
            class: "mt-6 border-l-2 pl-6 italic {class}",
            {children}
        }
    }
}

#[component]
pub fn InlineCode(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        code {
            class: "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold {class}",
            {children}
        }
    }
}

#[component]
pub fn Lead(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        p {
            class: "text-xl text-muted-foreground {class}",
            {children}
        }
    }
}

#[component]
pub fn Large(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "text-lg font-semibold {class}",
            {children}
        }
    }
}

#[component]
pub fn Small(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        small {
            class: "text-sm font-medium leading-none {class}",
            {children}
        }
    }
}

#[component]
pub fn Muted(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground {class}",
            {children}
        }
    }
}
