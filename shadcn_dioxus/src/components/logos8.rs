use dioxus::prelude::*;

#[component]
pub fn Logos8() -> Element {
    rsx! {
        section {
            class: "py-32",
            div {
                class: "container",
                div {
                    class: "flex flex-col items-center text-center",
                    h2 {
                        class: "text-3xl font-bold",
                        "Trusted by these companies"
                    }
                    p {
                        class: "text-muted-foreground mt-1",
                        "Used by the world\u{2019}s leading companies"
                    }
                    div {
                        class: "mt-8 flex flex-wrap items-center justify-center gap-x-8 gap-y-6 lg:gap-12",
                        img { class: "h-7 w-auto", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/vercel-wordmark.svg", alt: "Vercel logo", width: "109", height: "48" }
                        img { class: "h-5 w-auto", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/tailwind-wordmark.svg", alt: "Astro logo", width: "109", height: "48" }
                        img { class: "h-6 w-auto", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/supabase-wordmark.svg", alt: "Supabase logo", width: "109", height: "48" }
                        img { class: "h-5 w-auto", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/figma-wordmark.svg", alt: "Figma logo", width: "109", height: "48" }
                        img { class: "h-6 w-auto", src: "https://deifkwefumgah.cloudfront.net/shadcnblocks/block/logos/astro-wordmark.svg", alt: "Astro logo", width: "109", height: "48" }
                    }
                }
            }
        }
    }
}
