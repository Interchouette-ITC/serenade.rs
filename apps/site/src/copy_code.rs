//! Copy buttons on code excerpts. They are added after hydration, so the
//! prerendered HTML and the hydrated view tree stay identical, and pages without
//! JavaScript show no inert button.

use std::time::Duration;

use leptos::prelude::set_timeout;
use leptos::task::spawn_local;
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::Element;

const COPY_LABEL: &str = "Copy to clipboard";
const COPY_ICON: &str = r#"<svg viewBox="0 0 24 24" aria-hidden="true"><rect x="9" y="9" width="12" height="12" rx="2"/><path d="M5 15H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v1"/></svg>"#;
const COPIED_ICON: &str =
    r#"<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M4 12l5 5L20 6"/></svg>"#;
const RESET_AFTER: Duration = Duration::from_millis(1600);

/// Wraps every command excerpt (`pre.code-block`) in `<main>` with a
/// copy-to-clipboard button. Illustrations such as the home page tree are left alone.
pub fn add_copy_buttons() {
    let Some(window) = web_sys::window() else {
        return;
    };
    // The Clipboard API only exists in secure contexts (HTTPS or localhost).
    if !window.is_secure_context() {
        return;
    }
    let Some(document) = window.document() else {
        return;
    };
    let Ok(blocks) = document.query_selector_all("main pre.code-block") else {
        return;
    };
    for index in 0..blocks.length() {
        let Some(pre) = blocks.item(index).and_then(|node| node.dyn_into::<Element>().ok()) else {
            continue;
        };
        let (Some(parent), Ok(frame), Ok(button)) = (
            pre.parent_node(),
            document.create_element("div"),
            document.create_element("button"),
        ) else {
            continue;
        };
        frame.set_class_name("code-copy");
        button.set_class_name("code-copy__button");
        let _ = button.set_attribute("type", "button");
        set_state(&button, COPY_LABEL, COPY_ICON, false);
        if parent.insert_before(&frame, Some(&pre)).is_err()
            || frame.append_child(&pre).is_err()
            || frame.append_child(&button).is_err()
        {
            continue;
        }

        let target = button.clone();
        let on_click = Closure::<dyn Fn()>::new(move || {
            let text = pre.text_content().unwrap_or_default();
            let Some(window) = web_sys::window() else {
                return;
            };
            let copy = JsFuture::from(window.navigator().clipboard().write_text(&text));
            let target = target.clone();
            spawn_local(async move {
                let (label, icon) = if copy.await.is_ok() {
                    ("Copied", COPIED_ICON)
                } else {
                    ("Copy failed", COPY_ICON)
                };
                set_state(&target, label, icon, true);
                set_timeout(
                    move || set_state(&target, COPY_LABEL, COPY_ICON, false),
                    RESET_AFTER,
                );
            });
        });
        let _ = button.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
        on_click.forget();
    }
}

fn set_state(button: &Element, label: &str, icon: &str, done: bool) {
    let _ = button.set_attribute("aria-label", label);
    let _ = button.set_attribute("title", label);
    let _ = button.class_list().toggle_with_force("is-done", done);
    button.set_inner_html(icon);
}
