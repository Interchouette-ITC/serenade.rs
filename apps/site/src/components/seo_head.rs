use leptos::prelude::*;
use rangular_aot::HostCell;
use rangular_host::{Host, HostError, Value};

include!(concat!(env!("OUT_DIR"), "/rangular/seo_head_view.rs"));

#[component]
pub fn SeoHead(title: String, description: String, canonical: String) -> impl IntoView {
    seo_head_view(HostCell::new(SeoHeadHost {
        title,
        description,
        canonical,
    }))
}

struct SeoHeadHost {
    title: String,
    description: String,
    canonical: String,
}

impl Host for SeoHeadHost {
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
