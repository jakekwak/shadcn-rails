mod components;
mod pages;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/showcase")]
    Showcase {},
    #[route("/about3")]
    About3 {},
    #[route("/banner1")]
    Banner1 {},
    #[route("/blog7")]
    Blog7 {},
    #[route("/blogpost1")]
    Blogpost1 {},
    #[route("/careers4")]
    Careers4 {},
    #[route("/casestudies2")]
    Casestudies2 {},
    #[route("/casestudy8")]
    Casestudy8 {},
    #[route("/changelog1")]
    Changelog1 {},
    #[route("/codeexample1")]
    Codeexample1 {},
    #[route("/community1")]
    Community1 {},
    #[route("/compare7")]
    Compare7 {},
    #[route("/compliance1")]
    Compliance1 {},
    #[route("/contact7")]
    Contact7 {},
    #[route("/content1")]
    Content1 {},
    #[route("/cta10")]
    Cta10 {},
    #[route("/cta11")]
    Cta11 {},
    #[route("/download2")]
    Download2 {},
    #[route("/experience5")]
    Experience5 {},
    #[route("/faq1")]
    Faq1 {},
    #[route("/feature1")]
    Feature1 {},
    #[route("/feature13")]
    Feature13 {},
    #[route("/feature166")]
    Feature166 {},
    #[route("/feature17")]
    Feature17 {},
    #[route("/feature197")]
    Feature197 {},
    #[route("/feature2")]
    Feature2 {},
    #[route("/feature43")]
    Feature43 {},
    #[route("/feature51")]
    Feature51 {},
    #[route("/feature72")]
    Feature72 {},
    #[route("/feature73")]
    Feature73 {},
    #[route("/footer2")]
    Footer2 {},
    #[route("/gallery6")]
    Gallery6 {},
    #[route("/hero1")]
    Hero1 {},
    #[route("/hero115")]
    Hero115 {},
    #[route("/hero3")]
    Hero3 {},
    #[route("/hero45")]
    Hero45 {},
    #[route("/hero47")]
    Hero47 {},
    #[route("/hero7")]
    Hero7 {},
    #[route("/integration3")]
    Integration3 {},
    #[route("/list2")]
    List2 {},
    #[route("/login1")]
    Login1 {},
    #[route("/logos8")]
    Logos8 {},
    #[route("/navbar1")]
    Navbar1 {},
    #[route("/pricing2")]
    Pricing2 {},
    #[route("/pricing4")]
    Pricing4 {},
    #[route("/pricing6")]
    Pricing6 {},
    #[route("/resource1")]
    Resource1 {},
    #[route("/service1")]
    Service1 {},
    #[route("/services4")]
    Services4 {},
    #[route("/signup1")]
    Signup1 {},
    #[route("/stats8")]
    Stats8 {},
    #[route("/team1")]
    Team1 {},
    #[route("/testimonial10")]
    Testimonial10 {},
    #[route("/timeline9")]
    Timeline9 {},
    #[route("/waitlist1")]
    Waitlist1 {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Router::<Route> {}
    }
}

// ── Index page with links ──

#[component]
fn Home() -> Element {
    let blocks = vec![
        "about3", "banner1", "blog7", "blogpost1", "careers4", "casestudies2",
        "casestudy8", "changelog1", "codeexample1", "community1", "compare7",
        "compliance1", "contact7", "content1", "cta10", "cta11", "download2",
        "experience5", "faq1", "feature1", "feature13", "feature166", "feature17",
        "feature197", "feature2", "feature43", "feature51", "feature72", "feature73",
        "footer2", "gallery6", "hero1", "hero115", "hero3", "hero45", "hero47",
        "hero7", "integration3", "list2", "login1", "logos8", "navbar1",
        "pricing2", "pricing4", "pricing6", "resource1", "service1", "services4",
        "signup1", "stats8", "team1", "testimonial10", "timeline9", "waitlist1",
    ];
    rsx! {
        div { class: "container mx-auto max-w-4xl py-10 px-4",
            h1 { class: "text-4xl font-extrabold tracking-tight mb-2", "shadcn/ui Dioxus Blocks" }
            p { class: "text-muted-foreground mb-8",
                a { href: "/showcase", class: "underline hover:text-foreground", "View base components showcase" }
            }
            div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3",
                for name in blocks {
                    a {
                        href: "/{name}",
                        class: "rounded-lg border bg-card p-3 text-sm font-medium hover:bg-accent hover:text-accent-foreground transition-colors",
                        "{name}"
                    }
                }
            }
        }
    }
}

#[component]
fn Showcase() -> Element {
    rsx! { pages::showcase::ShowcasePage {} }
}

// ── Block routes ──

macro_rules! block_page {
    ($name:ident, $mod:ident, $comp:ident) => {
        #[component]
        fn $name() -> Element {
            rsx! {
                div { class: "min-h-screen",
                    div { class: "fixed top-4 left-4 z-50",
                        a {
                            href: "/",
                            class: "inline-flex items-center gap-1 rounded-md border bg-background px-3 py-1.5 text-sm shadow-sm hover:bg-accent",
                            "← Home"
                        }
                    }
                    components::$mod::$comp {}
                }
            }
        }
    };
}

block_page!(About3, about3, About3);
block_page!(Banner1, banner1, Banner1);
block_page!(Blog7, blog7, Blog7);
block_page!(Blogpost1, blogpost1, Blogpost1);
block_page!(Careers4, careers4, Careers4);
block_page!(Casestudies2, casestudies2, Casestudies2);
block_page!(Casestudy8, casestudy8, Casestudy8);
block_page!(Changelog1, changelog1, Changelog1);
block_page!(Codeexample1, codeexample1, Codeexample1);
block_page!(Community1, community1, Community1);
block_page!(Compare7, compare7, Compare7);
block_page!(Compliance1, compliance1, Compliance1);
block_page!(Contact7, contact7, Contact7);
block_page!(Content1, content1, Content1);
block_page!(Cta10, cta10, Cta10);
block_page!(Cta11, cta11, Cta11);
block_page!(Download2, download2, Download2);
block_page!(Experience5, experience5, Experience5);
block_page!(Faq1, faq1, Faq1);
block_page!(Feature1, feature1, Feature1);
block_page!(Feature13, feature13, Feature13);
block_page!(Feature166, feature166, Feature166);
block_page!(Feature17, feature17, Feature17);
block_page!(Feature197, feature197, Feature197);
block_page!(Feature2, feature2, Feature2);
block_page!(Feature43, feature43, Feature43);
block_page!(Feature51, feature51, Feature51);
block_page!(Feature72, feature72, Feature72);
block_page!(Feature73, feature73, Feature73);
block_page!(Footer2, footer2, Footer2);
block_page!(Gallery6, gallery6, Gallery6);
block_page!(Hero1, hero1, Hero1);
block_page!(Hero115, hero115, Hero115);
block_page!(Hero3, hero3, Hero3);
block_page!(Hero45, hero45, Hero45);
block_page!(Hero47, hero47, Hero47);
block_page!(Hero7, hero7, Hero7);
block_page!(Integration3, integration3, Integration3);
block_page!(List2, list2, List2);
block_page!(Login1, login1, Login1);
block_page!(Logos8, logos8, Logos8);
block_page!(Navbar1, navbar1, Navbar1);
block_page!(Pricing2, pricing2, Pricing2);
block_page!(Pricing4, pricing4, Pricing4);
block_page!(Pricing6, pricing6, Pricing6);
block_page!(Resource1, resource1, Resource1);
block_page!(Service1, service1, Service1);
block_page!(Services4, services4, Services4);
block_page!(Signup1, signup1, Signup1);
block_page!(Stats8, stats8, Stats8);
block_page!(Team1, team1, Team1);
block_page!(Testimonial10, testimonial10, Testimonial10);
block_page!(Timeline9, timeline9, Timeline9);
block_page!(Waitlist1, waitlist1, Waitlist1);
