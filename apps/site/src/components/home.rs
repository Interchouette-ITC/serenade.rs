use leptos::prelude::*;
use rangular_aot::HostCell;

use super::StaticHost;

include!(concat!(env!("OUT_DIR"), "/rangular/home_view.rs"));

#[component]
pub fn Home() -> impl IntoView {
    home_view(HostCell::new(StaticHost))
}
