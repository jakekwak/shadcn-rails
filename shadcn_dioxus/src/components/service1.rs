use dioxus::prelude::*;

#[component]
pub fn Service1() -> Element {
    rsx! {
        section {
            class: "pb-32",
            // Full Width Hero
            div {
                class: "bg-muted py-32",
                div {
                    class: "container text-center",
                    h1 {class: "text-4xl font-bold tracking-tight md:text-5xl lg:text-6xl",
                        "UX/UI Design"}
                }
            }

            // Intro Section
            div {
                class: "py-16",
                div {
                    class: "container",
                    div {
                        class: "mx-auto max-w-3xl space-y-8 text-left",
                        h2 {class: "text-3xl font-semibold tracking-tight md:text-4xl",
                            "User-Centered Design That Converts"}
                        p {class: "text-muted-foreground text-xl leading-relaxed",
                            "We believe that great design should be intuitive, accessible, and purposeful for every user who interacts with your product. Our UX/UI design approach focuses on understanding your users' needs, behaviors, and pain points to create interfaces that not only look beautiful but function seamlessly."}
                    }
                }
            }

            // Content Section
            div {
                class: "py-16",
                div {
                    class: "container",
                    div {
                        class: "prose prose-sm mx-auto max-w-3xl",
                        h2 {"Creating Meaningful Digital Experiences"}
                        p {"We combine user research, information architecture, and visual design to deliver experiences that drive engagement and conversions."}

                        p {"Through comprehensive user research and testing, we validate design decisions with real data. Our iterative design process ensures that every element serves a purpose and contributes to your business goals while providing an exceptional user experience."}

                        p {"We specialize in creating design systems that scale with your business, ensuring consistency across all touchpoints while maintaining flexibility for future growth and evolution."}

                        p {"Our collaborative approach involves stakeholders throughout the design process, from initial wireframes to final prototypes. This ensures alignment between business objectives and user needs, resulting in products that succeed in the market."}

                        p {"Every design decision is backed by research and testing, creating solutions that are not just visually appealing but strategically sound and user-validated."}

                        h2 {"Our UX/UI Design Services"}
                        ul {
                            li {"User research and persona development"}
                            li {"Information architecture and user journey mapping"}
                            li {"Wireframing and interactive prototyping"}
                            li {"Visual design and brand integration"}
                            li {"Usability testing and design validation"}
                            li {"Design system creation and documentation"}
                        }

                        h2 {"Strategic Design for Business Success"}
                        p {"Our design philosophy centers on creating interfaces that bridge the gap between user needs and business objectives. We understand that great UX/UI design is not just about aesthetics—it's about creating meaningful interactions that drive results."}

                        p {"From initial concept to final implementation, we ensure that every design element contributes to a cohesive user experience that reflects your brand values and supports your business goals. Our designs are optimized for performance, accessibility, and scalability across all devices and platforms."}
                    }
                }
            }
        }
    }
}
