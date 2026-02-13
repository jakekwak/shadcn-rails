use dioxus::prelude::*;

#[component]
pub fn Feature166() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "mb-24 flex flex-col items-center gap-6",
                    h1 {class: "text-center text-3xl font-semibold lg:max-w-3xl lg:text-5xl",
                        "Blocks built with Shadcn & Tailwind"}
                    p {class: "text-muted-foreground text-center text-lg font-medium md:max-w-4xl lg:text-xl",
                        "Finely crafted components built with React, Tailwind and Shadcn UI. Developers can copy and paste these blocks directly into their project."}
                }
                div {
                    class: "relative flex justify-center",
                    div {
                        class: "border-muted2 relative flex w-full flex-col border md:w-1/2 lg:w-full",
                        div {
                            class: "relative flex flex-col lg:flex-row",
                            div {
                                class: "border-muted2 flex flex-col justify-between border-b border-solid p-10 lg:w-3/5 lg:border-b-0 lg:border-r",
                                h2 {class: "text-xl font-semibold",
                                    "UI/UX Design"}
                                p {class: "text-muted-foreground",
                                    "Creating intuitive user experiences with modern interface design principles and user-centered methodologies."}
                                img {class: "mt-8 aspect-[1.5] h-full w-full object-cover lg:aspect-[2.4]", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-1.svg", alt: "UI/UX Design"}
                            }
                            div {
                                class: "flex flex-col justify-between p-10 lg:w-2/5",
                                h2 {class: "text-xl font-semibold",
                                    "Responsive Development"}
                                p {class: "text-muted-foreground",
                                    "Building websites that look and function perfectly across all devices and screen sizes."}
                                img {class: "mt-8 aspect-[1.45] h-full w-full object-cover", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-2.svg", alt: "Responsive Development"}
                            }
                        }
                        div {
                            class: "border-muted2 relative flex flex-col border-t border-solid lg:flex-row",
                            div {
                                class: "border-muted2 flex flex-col justify-between border-b border-solid p-10 lg:w-2/5 lg:border-b-0 lg:border-r",
                                h2 {class: "text-xl font-semibold",
                                    "Brand Integration"}
                                p {class: "text-muted-foreground",
                                    "Seamlessly incorporating your brand identity into every aspect of your website's design."}
                                img {class: "mt-8 aspect-[1.45] h-full w-full object-cover", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-3.svg", alt: "Brand Integration"}
                            }
                            div {
                                class: "flex flex-col justify-between p-10 lg:w-3/5",
                                h2 {class: "text-xl font-semibold",
                                    "Performance Optimization"}
                                p {class: "text-muted-foreground",
                                    "Ensuring fast loading times and smooth performance through optimized code and assets."}
                                img {class: "mt-8 aspect-[1.5] h-full w-full object-cover lg:aspect-[2.4]", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/placeholder-4.svg", alt: "Performance Optimization"}
                            }
                        }
                    }
                }
            }
        }
    }
}
