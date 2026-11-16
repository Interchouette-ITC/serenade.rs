use leptos::prelude::*;
use rangular_aot::HostCell;
use rangular_host::{Host, HostError, Value};
use serenade_view::{Document, DocumentAssets, DocumentSlot, RenderedDocument, ViewError};

use crate::{SitePage, components::StaticHost};

include!(concat!(env!("OUT_DIR"), "/rangular/document_view.rs"));
include!(concat!(env!("OUT_DIR"), "/rangular/head_view.rs"));

/// Route metadata rendered by the SEO head template.
#[derive(Clone, Debug)]
pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub canonical: String,
}

impl Host for PageMeta {
    fn get(&self, name: &str) -> Option<Value> {
        match name {
            "title" => Some(Value::from(self.title.as_str())),
            "description" => Some(Value::from(self.description.as_str())),
            "canonical" => Some(Value::from(self.canonical.as_str())),
            _ => None,
        }
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}

/// Renders one complete page document with Trunk's generated assets.
///
/// # Errors
///
/// Returns Serenade's rendering error.
pub fn render_document(
    page: &'static str,
    meta: PageMeta,
    assets: &DocumentAssets,
) -> Result<RenderedDocument, ViewError> {
    Document::new(
        move || head_view(HostCell::new(meta)),
        move || view! { <SitePage page /> },
    )
    .with_assets(assets)
    .render_with(|head: DocumentSlot, body: DocumentSlot| {
        document_view(HostCell::new(StaticHost), head, body)
    })
}
