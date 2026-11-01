use leptos::prelude::*;
use rangular_aot::HostCell;

use super::StaticHost;

include!(concat!(env!("OUT_DIR"), "/rangular/components_view.rs"));

#[component]
pub fn Components() -> impl IntoView {
    components_view(HostCell::new(StaticHost))
}
