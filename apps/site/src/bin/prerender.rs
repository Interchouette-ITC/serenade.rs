use anyhow::{Context, bail};
use serenade_site::PageMeta;
use serenade_view::{StaticDocumentWriter, TrunkAssetManifest, initialize_leptos_ssr};

fn main() -> anyhow::Result<()> {
    initialize_leptos_ssr()?;
    let manifest = TrunkAssetManifest::read_from_env()?;
    let [bundle] = manifest.wasm.as_slice() else {
        bail!("the site needs exactly one Trunk WASM bundle to hydrate");
    };
    if bundle.app_type != "main" || !bundle.inject_scripts {
        bail!("the site requires one main Trunk WASM bundle with script injection for hydration");
    }
    let assets = manifest
        .document_assets()
        .context("read Trunk's generated document assets")?;
    let writer = StaticDocumentWriter::from_env()?;

    for (route, page) in serenade_site::ROUTES {
        let document = serenade_site::render_document(page, page_metadata(route, page), &assets)
            .with_context(|| format!("render {route}"))?;
        let output = writer.write(route, document)?;
        println!("Prerendered {route} to {}", output.display());
    }

    Ok(())
}

fn page_metadata(route: &str, page: &str) -> PageMeta {
    let (title, description) = match page {
        "getting-started" => (
            "Getting started with Serenade",
            "Start exploring the pre-1.0 Serenade Rust framework from its source and examples.",
        ),
        "architecture" => (
            "Serenade architecture",
            "Learn how Serenade composes Rust applications with a kernel, components, and bundles.",
        ),
        "components" => (
            "Serenade components",
            "Explore the reusable components and crates in the Serenade Rust application framework.",
        ),
        _ => (
            "Serenade - Rust application framework",
            "Meet Serenade: a Rust application framework for building maintainable software from clear, reusable components.",
        ),
    };
    PageMeta {
        title: title.to_owned(),
        description: description.to_owned(),
        canonical: format!("https://serenade.rs{route}"),
    }
}
