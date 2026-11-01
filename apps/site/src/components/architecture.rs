use leptos::prelude::*;
use rangular_aot::HostCell;

use super::StaticHost;

include!(concat!(env!("OUT_DIR"), "/rangular/architecture_view.rs"));

#[component]
pub fn Architecture() -> impl IntoView {
    architecture_view(HostCell::new(StaticHost))
}
