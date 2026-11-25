use leptos::prelude::*;

mod components;
#[cfg(feature = "hydrate")]
pub mod copy_code;
#[cfg(feature = "ssr")]
mod document;

use components::{
    architecture::Architecture, components_page::Components as ComponentsPage,
    getting_started::GettingStarted, home::Home, site_footer::SiteFooter, site_header::SiteHeader,
    site_layout::site_layout,
};
#[cfg(feature = "ssr")]
pub use document::{PageMeta, render_document};

/// Prerendered routes and the page each one renders.
pub const ROUTES: [(&str, &str); 4] = [
    ("/", "home"),
    ("/getting-started/", "getting-started"),
    ("/architecture/", "architecture"),
    ("/components/", "components"),
];

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
pub(crate) fn SitePage(page: &'static str) -> impl IntoView {
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

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::{PageMeta, ROUTES, page_for_path, render_document};
    use serenade_view::DocumentAssets;
    use std::sync::Once;

    fn render(page: &'static str, meta: PageMeta, assets: &DocumentAssets) -> String {
        static INITIALIZE: Once = Once::new();
        INITIALIZE.call_once(|| {
            serenade_view::initialize_leptos_ssr().expect("initialize SSR executor");
        });
        render_document(page, meta, assets)
            .expect("render page document")
            .into_static_output()
    }

    fn meta(title: &str, description: &str, canonical: &str) -> PageMeta {
        PageMeta {
            title: title.to_owned(),
            description: description.to_owned(),
            canonical: canonical.to_owned(),
        }
    }

    fn render_page(page: &'static str) -> String {
        let html = render(
            page,
            meta("Title", "Description", "https://serenade.rs/"),
            &DocumentAssets::default(),
        );
        let start = html.find("<body>").expect("document body");
        html[start..].to_owned()
    }

    fn render_seo_head(title: &str, description: &str, canonical: &str) -> String {
        let html = render(
            "home",
            meta(title, description, canonical),
            &DocumentAssets::default(),
        );
        let start = html.find("<head>").expect("document head");
        let end = html.find("</head>").expect("document head end");
        html[start..end].to_owned()
    }

    #[test]
    fn documents_combine_template_head_page_body_and_trunk_assets_once() {
        let assets = DocumentAssets::new(
            "<link rel=\"stylesheet\" href=\"/base-1a2b.css\"><script type=\"module\">import init from '/serenade-site-3c4d.js';</script>",
            "",
        );
        for (route, page) in ROUTES {
            let canonical = format!("https://serenade.rs{route}");
            let html = render(page, meta("Title", "Description", &canonical), &assets);
            assert!(
                html.starts_with("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">")
            );
            assert!(html.contains("name=\"theme-color\""));
            assert!(html.contains("href=\"/favicon-32.png\""));
            assert!(html.contains(&format!("<link rel=\"canonical\" href=\"{canonical}\">")));
            assert_eq!(html.matches("/base-1a2b.css").count(), 1, "{html}");
            assert!(
                html.contains("'/serenade-site-3c4d.js';</script></head>"),
                "{html}"
            );
            assert!(html.contains("<body><div class=\"site-shell\">"), "{html}");
            assert!(html.ends_with("</body></html>"), "{html}");
            assert!(!html.contains("SERENADE_SSG"));
        }
    }

    #[test]
    fn shared_header_renders_responsive_logo_and_github_link_on_every_route() {
        for (_, page) in ROUTES {
            let html = render_page(page);
            let header = html
                .split("<header")
                .nth(1)
                .and_then(|header| header.split("</header>").next())
                .expect("shared header");
            for expected in [
                "<picture>",
                "media=\"(max-width: 1240px)\"",
                "srcset=\"/serenade-header-logo-round.png\"",
                "src=\"/serenade-header-logo.png\"",
                "alt=\"Serenade, Rust application framework\"",
                "class=\"header-cta\"",
                "href=\"https://github.com/Interchouette-ITC/Serenade\"",
                "View on GitHub",
            ] {
                assert!(header.contains(expected), "missing {expected} on {page}");
            }
        }
    }

    #[test]
    fn seo_head_renders_template_attributes_and_escapes_values() {
        let html = render_seo_head(
            "Serenade & \"Rust\" <framework>",
            "Components & \"bundles\"",
            "https://serenade.rs/components/",
        );
        for expected in [
            "<title>Serenade &amp; \"Rust\" &lt;framework&gt;</title>",
            "property=\"og:title\"",
            "property=\"og:description\"",
            "property=\"og:url\"",
            "content=\"Serenade &amp; &quot;Rust&quot; &lt;framework&gt;\"",
            "content=\"Components &amp; &quot;bundles&quot;\"",
            "href=\"https://serenade.rs/components/\"",
            "content=\"https://serenade.rs/components/\"",
            "content=\"https://serenade.rs/serenade.rs.png\"",
            "name=\"twitter:card\" content=\"summary_large_image\"",
            "type=\"application/ld+json\"",
            "\"@context\":\"https://schema.org\"",
        ] {
            assert!(
                html.contains(expected),
                "missing SEO output: {expected}\n{html}"
            );
        }
        assert_eq!(html.matches("<title>").count(), 1);
        assert!(!html.contains("{{"));
    }

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
                html.contains("href=\"https://docs.serenade.rs\""),
                "documentation link absent on {page}"
            );
        }
    }

    #[test]
    fn getting_started_explains_the_documentation_mcp() {
        let html = render_page("getting-started");
        for expected in [
            "Source-backed help for Serenade and Rangular",
            "serenade-rangular-mcp",
            "--profile",
            "embedded source version and revision",
            "not an application build or AOT check",
            "requests approval",
            "https://docs.serenade.rs/how-to/agent-mcp/",
        ] {
            assert!(html.contains(expected), "missing {expected}");
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
        assert_eq!(html.matches("https://crates.io/crates/serenade-").count(), 42);
        assert!(html.contains("https://docs.rs/serenade-framework"));
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
        assert!(html.contains("cargo add serenade-framework --rename serenade"));
        assert!(html.contains("git clone https://github.com/Interchouette-ITC/Serenade.git"));
    }

    #[test]
    fn footer_credits_the_site_stack_and_keeps_api_docs_link() {
        let html = render_page("home");
        for credit in [
            "https://leptos.dev",
            "https://rangular.rust.contact",
            "https://serenade.rust.contact",
            "page templates and nested SCSS",
        ] {
            assert!(html.contains(credit), "missing footer credit: {credit}");
        }
        assert!(!html.contains("No analytics or forms"));
    }

    #[test]
    fn rangular_templates_preserve_inline_spaces_and_preformatted_commands() {
        let home = render_page("home");
        assert!(home.contains("Meet<!> <span>Serenade."));
        assert!(home.contains("https://leptos.dev"));
        assert!(home.contains("Kernel &amp; bundles"));
        assert!(home.contains("Your application\n├── Kernel        the conductor"));

        let getting_started = render_page("getting-started");
        assert!(getting_started.contains("make test\nmake demo"));
    }
}
