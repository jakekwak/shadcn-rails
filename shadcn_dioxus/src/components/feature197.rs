use dioxus::prelude::*;
use crate::components::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn Feature197() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container mx-auto",
                div {
                    class: "mb-12 flex w-full items-start justify-between gap-12",
                    div {
                        class: "w-full md:w-1/2",
                        Accordion {
                            class: "w-full",
                            accordion_type: "single",
                            default_value: "item-1",
                            AccordionItem {
                                class: "transition-opacity hover:opacity-80",
                                value: "item-1",
                                AccordionTrigger {
                                    class: "no-underline cursor-pointer py-5 transition",
                                    h4 {
                                        class: "text-xl font-semibold",
                                        "Ready-to-Use UI Blocks"
                                    }
                                }
                                AccordionContent {
                                    p {
                                        class: "text-muted-foreground text-base",
                                        "Browse through our extensive collection of pre-built UI blocks designed with shadcn/ui. Each block is carefully crafted to be responsive, accessible, and easily customizable. Simply copy and paste the code into your project."
                                    }
                                    div {
                                        class: "mt-4 md:hidden",
                                        img {
                                            class: "h-full max-h-80 w-full rounded-md object-cover",
                                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                                            alt: "Ready-to-Use UI Blocks",
                                        }
                                    }
                                }
                            }
                            AccordionItem {
                                class: "transition-opacity hover:opacity-80",
                                value: "item-2",
                                AccordionTrigger {
                                    class: "no-underline cursor-pointer py-5 transition",
                                    h4 {
                                        class: "text-xl font-semibold",
                                        "Tailwind CSS & TypeScript"
                                    }
                                }
                                AccordionContent {
                                    p {
                                        class: "text-muted-foreground text-base",
                                        "Built with Tailwind CSS for rapid styling and TypeScript for type safety. Our blocks leverage the full power of Tailwind\u{2019}s utility classes while maintaining clean, type-safe code that integrates seamlessly with your Next.js projects."
                                    }
                                    div {
                                        class: "mt-4 md:hidden",
                                        img {
                                            class: "h-full max-h-80 w-full rounded-md object-cover",
                                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-2.svg",
                                            alt: "Tailwind CSS & TypeScript",
                                        }
                                    }
                                }
                            }
                            AccordionItem {
                                class: "transition-opacity hover:opacity-80",
                                value: "item-3",
                                AccordionTrigger {
                                    class: "no-underline cursor-pointer py-5 transition",
                                    h4 {
                                        class: "text-xl font-semibold",
                                        "Dark Mode & Customization"
                                    }
                                }
                                AccordionContent {
                                    p {
                                        class: "text-muted-foreground text-base",
                                        "Every block supports dark mode out of the box and can be customized to match your brand. Modify colors, spacing, and typography using Tailwind\u{2019}s configuration. The shadcn/ui theming system makes it easy to maintain consistency across your site."
                                    }
                                    div {
                                        class: "mt-4 md:hidden",
                                        img {
                                            class: "h-full max-h-80 w-full rounded-md object-cover",
                                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-3.svg",
                                            alt: "Dark Mode & Customization",
                                        }
                                    }
                                }
                            }
                            AccordionItem {
                                class: "transition-opacity hover:opacity-80",
                                value: "item-4",
                                AccordionTrigger {
                                    class: "no-underline cursor-pointer py-5 transition",
                                    h4 {
                                        class: "text-xl font-semibold",
                                        "Accessibility First"
                                    }
                                }
                                AccordionContent {
                                    p {
                                        class: "text-muted-foreground text-base",
                                        "All blocks are built with accessibility in mind, following WCAG guidelines. They include proper ARIA labels, keyboard navigation support, and semantic HTML structure. Ensure your website is usable by everyone without extra effort."
                                    }
                                    div {
                                        class: "mt-4 md:hidden",
                                        img {
                                            class: "h-full max-h-80 w-full rounded-md object-cover",
                                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-4.svg",
                                            alt: "Accessibility First",
                                        }
                                    }
                                }
                            }
                            AccordionItem {
                                class: "transition-opacity hover:opacity-80",
                                value: "item-5",
                                AccordionTrigger {
                                    class: "no-underline cursor-pointer py-5 transition",
                                    h4 {
                                        class: "text-xl font-semibold",
                                        "Modern Development Stack"
                                    }
                                }
                                AccordionContent {
                                    p {
                                        class: "text-muted-foreground text-base",
                                        "Built for modern web development with React 18, Next.js 14, and the latest shadcn/ui components. Take advantage of React Server Components, TypeScript strict mode, and other cutting-edge features while maintaining excellent performance."
                                    }
                                    div {
                                        class: "mt-4 md:hidden",
                                        img {
                                            class: "h-full max-h-80 w-full rounded-md object-cover",
                                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-5.svg",
                                            alt: "Modern Development Stack",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "bg-muted relative m-auto hidden w-1/2 overflow-hidden rounded-xl md:block",
                        img {
                            class: "aspect-4/3 rounded-md object-cover pl-4",
                            alt: "Feature preview",
                            src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg",
                        }
                    }
                }
            }
        }
    }
}
