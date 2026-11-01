use leptos::prelude::*;
use rangular_aot::HostCell;

use super::StaticHost;

include!(concat!(
    env!("OUT_DIR"),
    "/rangular/getting_started_view.rs"
));

#[component]
pub fn GettingStarted() -> impl IntoView {
    getting_started_view(HostCell::new(StaticHost))
}
