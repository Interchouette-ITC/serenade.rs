#[cfg(feature = "hydrate")]
fn main() {
    leptos::mount::hydrate_body(serenade_site::App);
    serenade_site::copy_code::add_copy_buttons();
}

#[cfg(not(feature = "hydrate"))]
fn main() {}
