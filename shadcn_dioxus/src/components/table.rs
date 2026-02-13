use dioxus::prelude::*;

#[component]
pub fn Table(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "relative w-full overflow-auto",
            table {
                class: "w-full caption-bottom text-sm {class}",
                {children}
            }
        }
    }
}

#[component]
pub fn TableHeader(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        thead {
            class: "[&_tr]:border-b {class}",
            {children}
        }
    }
}

#[component]
pub fn TableBody(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        tbody {
            class: "[&_tr:last-child]:border-0 {class}",
            {children}
        }
    }
}

#[component]
pub fn TableFooter(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        tfoot {
            class: "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 {class}",
            {children}
        }
    }
}

#[component]
pub fn TableRow(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        tr {
            class: "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted {class}",
            {children}
        }
    }
}

#[component]
pub fn TableHead(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        th {
            class: "h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {class}",
            {children}
        }
    }
}

#[component]
pub fn TableCell(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        td {
            class: "p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {class}",
            {children}
        }
    }
}

#[component]
pub fn TableCaption(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        caption {
            class: "mt-4 text-sm text-muted-foreground {class}",
            {children}
        }
    }
}
