use dioxus::prelude::*;
use crate::components::{Badge};

#[component]
pub fn Compliance1() -> Element {
    rsx! {
        section {
            class: "bg-muted/50 py-32",
            div {
                class: "container",
                div {
                    class: "grid gap-9 lg:grid-cols-2",
                    div {
                        class: "flex flex-col gap-5",
                        Badge {
                            class: "bg-background gap-1.5", variant: "outline",
                            span {class: "size-1.5 rounded-full bg-green-500"}
                            "Compliance"
                        }
                        h1 {class: "text-balance text-4xl font-medium lg:text-5xl",
                            "Complete Compliance & Security Readiness"}
                        p {class: "text-muted-foreground text-lg",
                            "Stay compliant with privacy and healthcare regulations. Our platform meets GDPR and HIPAA requirements, providing data protection and compliance monitoring for regulated industries."}
                        div {
                            class: "flex items-center gap-6",
                            img {class: "h-22 opacity-50 grayscale md:h-28", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/compliance/GDPR.svg", alt: "GDPR"}
                            img {class: "h-22 opacity-60 grayscale md:h-28", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/compliance/CCPA.svg", alt: "ISO-27001"}
                        }
                    }
                    div {
                        class: "border-border bg-background rounded-2xl border",
                        div {
                            class: "border-border relative overflow-hidden border-b p-6 lg:px-8 lg:py-11",
                            div {
                                h2 {class: "text-xl font-medium lg:text-2xl",
                                    "Automated audit trails"}
                                p {class: "text-muted-foreground mt-2 w-3/4 pr-10 text-sm md:text-base",
                                    "Every action is logged and timestamped with immutable audit trails for complete regulatory compliance."}
                            }
                            img {class: "text-muted-foreground absolute -bottom-7 right-4 size-24 opacity-80 grayscale lg:right-8 lg:size-32", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/compliance/ISO-27001.svg", alt: "ISO-27001"}
                        }
                        div {
                            class: "relative overflow-hidden p-6 lg:px-8 lg:py-11",
                            div {
                                h2 {class: "text-xl font-medium lg:text-2xl",
                                    "Compliance monitoring"}
                                p {class: "text-muted-foreground mt-2 w-3/4 pr-10 text-sm md:text-base",
                                    "Real-time monitoring ensures continuous compliance with industry standards and regulations."}
                            }
                            img {class: "text-muted-foreground absolute -bottom-7 right-4 size-24 opacity-80 grayscale lg:right-8 lg:size-32", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/compliance/ISO-27017.svg", alt: "ISO-27001"}
                        }
                        div {
                            class: "border-border relative overflow-hidden border-t p-6 lg:px-8 lg:py-11",
                            div {
                                h2 {class: "text-xl font-medium lg:text-2xl",
                                    "Regulatory reporting"}
                                p {class: "text-muted-foreground mt-2 w-3/4 pr-10 text-sm md:text-base",
                                    "Generate compliance reports automatically to meet regulatory requirements and audit demands."}
                            }
                            img {class: "text-muted-foreground absolute -bottom-7 right-4 size-24 opacity-80 grayscale lg:right-8 lg:size-32", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/compliance/ISO-27018.svg", alt: "ISO-27001"}
                        }
                    }
                }
            }
        }
    }
}
