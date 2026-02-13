use dioxus::prelude::*;
use crate::components::Table;

#[component]
pub fn Compare7() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                h2 { class: "mb-4 text-center text-4xl font-semibold", "Compare Us" }
                p { class: "text-muted-foreground mb-8 text-center",
                    "A modern framework for building websites that is better than the competition."
                }
                div {
                    class: "mx-auto max-w-3xl overflow-x-auto",
                    Table {
                        class: "rounded border text-left shadow-lg",
                        thead {
                            class: "[&_tr]:border-b",
                            tr {
                                class: "border-b transition-colors",
                                th { class: "h-12 px-6 py-4 text-left align-middle font-medium text-muted-foreground" }
                                th { class: "h-12 px-6 py-4 text-left align-middle font-semibold bg-muted", "Shadcn" }
                                th { class: "h-12 px-6 py-4 text-left align-middle font-semibold", "Bootstrap" }
                            }
                        }
                        tbody {
                            class: "[&_tr:last-child]:border-0 text-foreground",
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Design System" }
                                td { class: "bg-muted px-6 py-4", "Modern, Utility-first" }
                                td { class: "px-6 py-4", "Classic, Component-based" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Customization" }
                                td { class: "bg-muted px-6 py-4", "Highly customizable" }
                                td { class: "px-6 py-4", "Limited by default" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Dark Mode" }
                                td { class: "bg-muted px-6 py-4", "Built-in" }
                                td { class: "px-6 py-4", "Requires extra setup" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "TypeScript Support" }
                                td { class: "bg-muted px-6 py-4", "First-class" }
                                td { class: "px-6 py-4", "Partial" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Accessibility" }
                                td { class: "bg-muted px-6 py-4", "Focus on a11y" }
                                td { class: "px-6 py-4", "Basic" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Component Count" }
                                td { class: "bg-muted px-6 py-4", "30+" }
                                td { class: "px-6 py-4", "25+" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "License" }
                                td { class: "bg-muted px-6 py-4", "MIT" }
                                td { class: "px-6 py-4", "MIT" }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Premium Components" }
                                td { class: "bg-muted px-6 py-4", "Available" }
                                td {
                                    class: "relative px-6 py-4",
                                    CssTooltip {
                                        label: "Not included",
                                        label_class: "cursor-pointer underline decoration-dotted",
                                        title: "Premium Only",
                                        body: "Some advanced components are only available in paid versions or require third-party libraries.",
                                    }
                                }
                            }
                            tr {
                                class: "border-b transition-colors hover:bg-muted/50",
                                td { class: "px-6 py-4", "Figma Kit" }
                                td { class: "bg-muted px-6 py-4", "Yes" }
                                td {
                                    class: "relative px-6 py-4",
                                    CssTooltip {
                                        label: "No",
                                        label_class: "text-muted-foreground cursor-pointer underline decoration-dotted",
                                        title: "Figma Kit Unavailable",
                                        body: "Bootstrap does not provide an official Figma kit, but community kits may exist.",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Pure CSS tooltip — hidden by default, shown on hover
#[component]
fn CssTooltip(
    label: String,
    #[props(default)] label_class: String,
    title: String,
    body: String,
) -> Element {
    rsx! {
        span {
            class: "group relative inline-block",
            span {
                class: "{label_class}",
                "{label}"
            }
            // Tooltip popup — invisible until group hover
            div {
                class: "pointer-events-none absolute bottom-full left-1/2 z-50 mb-2 -translate-x-1/2 opacity-0 transition-opacity group-hover:pointer-events-auto group-hover:opacity-100",
                div {
                    class: "w-64 rounded-md border bg-popover px-3 py-2 text-sm text-popover-foreground shadow-md",
                    span { class: "mb-1 block font-semibold", "{title}" }
                    "{body}"
                }
            }
        }
    }
}
