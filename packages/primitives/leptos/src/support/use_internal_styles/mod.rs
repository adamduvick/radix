use crate::support::compose_refs::use_composed_refs;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

/// Apply static CSS properties via `style.setProperty()` on the DOM node,
/// avoiding conflicts with user-provided `attr:style` which overwrites
/// all inline styles via `setAttribute`.
///
/// Returns a composed ref that should be passed as `node_ref` to the element.
pub fn use_internal_styles(
    node_ref: AnyNodeRef,
    properties: &[(&'static str, &'static str)],
) -> AnyNodeRef {
    let properties: Vec<(&'static str, &'static str)> = properties.to_vec();
    use_internal_styles_effect(node_ref, move |style| {
        for (name, value) in &properties {
            let _ = style.set_property(name, value);
        }
    })
}

/// Apply reactive/conditional CSS properties via `style.setProperty()` on
/// the DOM node. The closure re-runs whenever any signal it reads changes.
///
/// Returns a composed ref that should be passed as `node_ref` to the element.
pub fn use_internal_styles_effect(
    node_ref: AnyNodeRef,
    apply: impl Fn(&web_sys::CssStyleDeclaration) + Send + Sync + 'static,
) -> AnyNodeRef {
    let local_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, local_ref]);

    Effect::new(move |_| {
        if let Some(node) = local_ref.get() {
            let el: &web_sys::HtmlElement = node.unchecked_ref();
            apply(&el.style());
        }
    });

    composed_ref
}
