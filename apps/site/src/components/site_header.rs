use leptos::prelude::*;
use rangular_aot::HostCell;

use super::StaticHost;

include!(concat!(env!("OUT_DIR"), "/rangular/site_header_view.rs"));

#[component]
pub fn SiteHeader() -> impl IntoView {
    site_header_view(HostCell::new(StaticHost))
}
