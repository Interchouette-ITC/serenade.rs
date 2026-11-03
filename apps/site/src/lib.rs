use leptos::prelude::*;

mod components;

use components::{
    architecture::Architecture, components::Components as ComponentsPage,
    getting_started::GettingStarted, home::Home, site_footer::SiteFooter, site_header::SiteHeader,
    site_layout::site_layout,
};

#[cfg(feature = "hydrate")]
#[component]
pub fn App() -> impl IntoView {
    let path = web_sys::window()
        .expect("browser window is available during hydration")
        .location()
        .pathname()
        .expect("browser pathname is available during hydration");
    view! { <SitePage page=page_for_path(&path) /> }
}

#[component]
fn SitePage(page: &'static str) -> impl IntoView {
    site_layout(Box::new(move || {
        view! {
            <SiteHeader />
            <main>
                {match page {
                    "getting-started" => view! { <GettingStarted /> }.into_any(),
                    "architecture" => view! { <Architecture /> }.into_any(),
                    "components" => view! { <ComponentsPage /> }.into_any(),
                    _ => view! { <Home /> }.into_any(),
                }}
            </main>
            <SiteFooter />
        }
        .into_any()
    }))
}

#[cfg(any(feature = "hydrate", test))]
fn page_for_path(path: &str) -> &'static str {
    match path.trim_matches('/') {
        "getting-started" => "getting-started",
        "architecture" => "architecture",
        "components" => "components",
        _ => "home",
    }
}

#[cfg(feature = "ssr")]
pub fn render_page(page: &'static str) -> String {
    let owner = Owner::new_root(Some(std::sync::Arc::new(
        hydration_context::SsrSharedContext::new(),
    )));
    owner.with(|| view! { <SitePage page /> }.to_html())
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::{page_for_path, render_page};

    #[test]
    fn routes_resolve_to_the_expected_pages() {
        assert_eq!(page_for_path("/"), "home");
        assert_eq!(page_for_path("/getting-started/"), "getting-started");
        assert_eq!(page_for_path("/architecture"), "architecture");
        assert_eq!(page_for_path("/components/"), "components");
    }

    #[test]
    fn each_page_prerenders_product_content() {
        let cases = [
            ("home", "Meet"),
            ("getting-started", "Get to know"),
            ("architecture", "Four parts"),
            ("components", "curiosity"),
        ];
        for (page, expected) in cases {
            let html = render_page(page);
            assert!(html.contains(expected), "missing {expected} on {page}");
            assert!(
                html.contains("Getting started"),
                "navigation absent on {page}"
            );
            assert!(
                html.contains("href=\"/docs/\""),
                "documentation link absent on {page}"
            );
        }
    }

    #[test]
    fn architecture_explains_symfony_inspiration_without_claiming_compatibility() {
        let html = render_page("architecture");
        for detail in [
            "What if Symfony",
            "Rust-native implementation",
            "not a PHP port",
            "Kernel",
            "Bundles",
            "Components",
            "Adapters",
            "Cargo remains the package manager",
            "actively evolving",
            "production-ready",
            "/components/",
            "/getting-started/",
        ] {
            assert!(
                html.contains(detail),
                "missing architecture detail: {detail}"
            );
        }
    }

    #[test]
    fn home_architecture_lists_all_four_parts() {
        let html = render_page("home");
        for part in ["Kernel", "Bundles", "Components", "Adapters"] {
            assert!(html.contains(part), "missing architecture part: {part}");
        }
        assert_eq!(html.matches("├──").count(), 3);
        assert_eq!(html.matches("└──").count(), 1);
    }

    #[test]
    fn component_catalog_lists_all_crates_and_api_docs_guide() {
        let html = render_page("components");
        assert_eq!(html.matches("/crates/").count(), 40);
        assert!(html.contains("Pre-1.0"));
        assert!(html.contains("docs/api-rust/README.md"));
        assert!(!html.contains("github.io"));
    }

    #[test]
    fn getting_started_documents_all_supported_front_modes() {
        let html = render_page("getting-started");
        for mode in ["make html", "make spa", "make ssg", "make universal"] {
            assert!(html.contains(mode), "missing mode command: {mode}");
        }
        assert!(html.contains("demo-spa/dist"));
        assert!(html.contains("demo-ssg/dist"));
        assert!(html.contains("/cms/hello"));
        assert!(html.contains("127.0.0.1:8080"));
    }

    #[test]
    fn footer_credits_the_site_stack_and_keeps_api_docs_link() {
        let html = render_page("home");
        for credit in [
            "https://leptos.dev",
            "https://github.com/Interchouette-ITC/rangular",
            "https://github.com/Interchouette-ITC/Serenade",
            "page templates and nested SCSS",
            "/docs/reference/api-docs/",
        ] {
            assert!(html.contains(credit), "missing footer credit: {credit}");
        }
        assert!(!html.contains("No analytics or forms"));
    }

    #[test]
    fn rangular_templates_preserve_inline_spaces_and_preformatted_commands() {
        let home = render_page("home");
        assert!(home.contains("Meet<!> <span>Serenade."));
        assert!(home.contains("Built with<!> <a href=\"https://leptos.dev\">"));
        assert!(home.contains("Kernel &amp; bundles"));
        assert!(home.contains("Your application\n├── Kernel        the conductor"));

        let getting_started = render_page("getting-started");
        assert!(getting_started.contains("make test\nmake demo"));
    }
}
