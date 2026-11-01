#[cfg(feature = "hydrate")]
fn main() {
    leptos::mount::hydrate_body(serenade_site::App);
}

#[cfg(not(feature = "hydrate"))]
fn main() {}
