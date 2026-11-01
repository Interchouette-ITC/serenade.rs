use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};

fn main() -> anyhow::Result<()> {
    let app = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dist = app.join("dist");
    let shell_path = dist.join("index.html");
    let shell = fs::read_to_string(&shell_path)
        .with_context(|| format!("read Trunk output {}", shell_path.display()))?;

    for (route, page) in [
        ("", "home"),
        ("getting-started", "getting-started"),
        ("architecture", "architecture"),
        ("components", "components"),
    ] {
        let mut html = shell.clone();
        let content_marker = "<!--SERENADE_SSG_CONTENT-->";
        if html.matches(content_marker).count() != 1 {
            bail!("Trunk output must contain exactly one {content_marker} marker");
        }
        html = html.replacen(content_marker, &serenade_site::render_page(page), 1);
        let (title, description) = page_metadata(page);
        html = html.replace(
            "<title>Serenade - Rust application framework</title>",
            &format!("<title>{title}</title>"),
        );
        html = html.replace(
            "content=\"Meet Serenade: a Rust application framework for building maintainable software from clear, reusable components.\"",
            &format!("content=\"{description}\""),
        );

        let output = if route.is_empty() {
            dist.join("index.html")
        } else {
            let directory = dist.join(route);
            fs::create_dir_all(&directory)
                .with_context(|| format!("create route directory {}", directory.display()))?;
            directory.join("index.html")
        };
        fs::write(&output, serenade_minify::minify_html(&html)?)
            .with_context(|| format!("write prerendered page {}", output.display()))?;
        println!("Prerendered /{route} to {}", output.display());
    }

    Ok(())
}

fn page_metadata(page: &str) -> (&'static str, &'static str) {
    match page {
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
    }
}
