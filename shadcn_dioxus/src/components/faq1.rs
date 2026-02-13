use dioxus::prelude::*;
use crate::components::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn Faq1() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container max-w-3xl",
                h1 { class: "mb-4 text-3xl font-semibold md:mb-11 md:text-4xl",
                    "Frequently asked questions"
                }
                Accordion {
                    accordion_type: "single",
                    AccordionItem {
                        value: "item-0",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "What is a FAQ?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "A FAQ is a list of frequently asked questions and answers on a particular topic."
                        }
                    }
                    AccordionItem {
                        value: "item-1",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "What is the purpose of a FAQ?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "The purpose of a FAQ is to provide answers to common questions and help users find the information they need quickly and easily."
                        }
                    }
                    AccordionItem {
                        value: "item-2",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "How do I create a FAQ?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "To create a FAQ, you need to compile a list of common questions and answers on a particular topic and organize them in a clear and easy-to-navigate format."
                        }
                    }
                    AccordionItem {
                        value: "item-3",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "What are the benefits of a FAQ?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "The benefits of a FAQ include providing quick and easy access to information, reducing the number of support requests, and improving the overall user experience."
                        }
                    }
                    AccordionItem {
                        value: "item-4",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "How should I organize my FAQ?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "You should organize your FAQ in a logical manner, grouping related questions together and ordering them from most basic to more advanced topics."
                        }
                    }
                    AccordionItem {
                        value: "item-5",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "How long should FAQ answers be?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "FAQ answers should be concise and to the point, typically a few sentences or a short paragraph is sufficient for most questions."
                        }
                    }
                    AccordionItem {
                        value: "item-6",
                        AccordionTrigger { class: "font-semibold hover:no-underline",
                            "Should I include links in my FAQ?"
                        }
                        AccordionContent { class: "text-muted-foreground",
                            "Yes, including links to more detailed information or related resources can be very helpful for users who want to learn more about a particular topic."
                        }
                    }
                }
            }
        }
    }
}
