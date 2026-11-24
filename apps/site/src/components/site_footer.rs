use leptos::prelude::*;
use rangular::aot::HostCell;

use super::StaticHost;

include!(concat!(env!("OUT_DIR"), "/rangular/site_footer_view.rs"));

#[component]
pub fn SiteFooter() -> impl IntoView {
    site_footer_view(HostCell::new(StaticHost))
}
