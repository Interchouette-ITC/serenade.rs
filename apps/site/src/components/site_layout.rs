use leptos::prelude::*;
use rangular_aot::HostCell;

use super::StaticHost;

include!(concat!(env!("OUT_DIR"), "/rangular/site_layout_view.rs"));

pub(crate) fn site_layout(children: Children) -> impl IntoView {
    site_layout_view(HostCell::new(StaticHost), children)
}
