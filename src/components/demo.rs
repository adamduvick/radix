use leptos::prelude::*;
use pith_ui::scroll_area::{
    Orientation, ScrollArea, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaType,
    ScrollAreaViewport,
};
use pith_ui::separator::Separator;
use pith_ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use wasm_bindgen::JsCast;

/// Extracts the content between `// #region demo` and `// #endregion demo` markers.
pub fn extract_demo(source: &str) -> String {
    let start_marker = "// #region demo";
    let end_marker = "// #endregion demo";

    let start = source
        .find(start_marker)
        .map(|i| i + start_marker.len())
        .unwrap_or(0);
    let end = source.find(end_marker).unwrap_or(source.len());

    source[start..end].trim().to_string()
}

/// Calls `Prism.highlightElement(el)` if Prism is loaded.
fn highlight_code(el: &web_sys::Element) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(prism) = js_sys::Reflect::get(&window, &"Prism".into()) else {
        return;
    };
    if prism.is_undefined() {
        return;
    }
    let Ok(func) = js_sys::Reflect::get(&prism, &"highlightElement".into()) else {
        return;
    };
    if let Some(func) = func.dyn_ref::<js_sys::Function>() {
        let _ = func.call1(&wasm_bindgen::JsValue::NULL, el);
    }
}

const TAB_TRIGGER: &str = "px-3 py-1.5 text-sm font-medium rounded-md transition-colors \
    text-slate-500 hover:text-slate-700 \
    data-[state=active]:bg-white data-[state=active]:text-accent-700 data-[state=active]:shadow-sm";

/// A tabbed demo viewer: "Preview" renders the live component, "Code" shows the source.
#[component]
pub fn DemoTabs(#[prop(into)] source: String, children: ChildrenFn) -> impl IntoView {
    let code_ref = NodeRef::<leptos::html::Code>::new();
    let children = StoredValue::new(children);
    let source = StoredValue::new(source);

    Effect::new(move |_| {
        if let Some(el) = code_ref.get() {
            highlight_code(&el);
        }
    });

    view! {
        <Tabs default_value="preview".to_string() attr:class="rounded-lg border border-slate-200 overflow-hidden">
            <TabsList attr:class="flex gap-1 bg-slate-100 p-1 border-b border-slate-200">
                <TabsTrigger value="preview".to_string() attr:class=TAB_TRIGGER>
                    "Preview"
                </TabsTrigger>
                <TabsTrigger value="code".to_string() attr:class=TAB_TRIGGER>
                    "Code"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="preview".to_string() attr:class="p-6 bg-white">
                {children.get_value()()}
            </TabsContent>
            <TabsContent value="code".to_string() force_mount=true attr:class="data-[state=inactive]:hidden bg-code">
                <ScrollArea r#type=ScrollAreaType::Auto attr:class="relative bg-code">
                    <ScrollAreaViewport attr:class="max-h-[680px]">
                        <pre class="m-0 bg-code p-4 text-sm leading-relaxed">
                            <code node_ref=code_ref class="language-rust font-mono">
                                {source.get_value()}
                            </code>
                        </pre>
                    </ScrollAreaViewport>
                    <ScrollAreaScrollbar
                        orientation=Orientation::Vertical
                        attr:class="flex w-2.5 touch-none select-none bg-transparent p-0.5 transition-colors"
                    >
                        <ScrollAreaThumb attr:class="relative flex-1 rounded-full bg-slate-500 hover:bg-slate-400 transition-colors">" "</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                    <ScrollAreaScrollbar
                        orientation=Orientation::Horizontal
                        attr:class="flex h-2.5 touch-none select-none bg-transparent p-0.5 transition-colors"
                    >
                        <ScrollAreaThumb attr:class="relative flex-1 rounded-full bg-slate-500 hover:bg-slate-400 transition-colors">" "</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                </ScrollArea>
            </TabsContent>
        </Tabs>
    }
}

/// A code example block.
#[component]
pub fn CodeBlock(code: &'static str) -> impl IntoView {
    view! {
        <pre class="overflow-x-auto rounded-lg bg-slate-900 p-4 text-sm leading-relaxed">
            <code class="font-mono text-slate-100">{code}</code>
        </pre>
    }
}

/// A badge showing a component feature.
#[component]
pub fn FeatureBadge(text: &'static str) -> impl IntoView {
    view! {
        <span class="inline-flex items-center rounded-full bg-accent-50 px-2.5 py-0.5 text-xs font-medium text-accent-700">
            {text}
        </span>
    }
}

/// Header for a component documentation page.
#[component]
pub fn ComponentPageHeader(
    title: &'static str,
    description: &'static str,
    #[prop(default = &[])] features: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div class="mb-10">
            <h1 class="text-3xl font-bold tracking-tight text-slate-900">{title}</h1>
            <p class="mt-2 text-lg text-slate-600">{description}</p>
            {(!features.is_empty()).then(|| view! {
                <div class="mt-4 flex flex-wrap gap-2">
                    {features.iter().map(|f| view! { <FeatureBadge text=f /> }).collect_view()}
                </div>
            })}
            <Separator
                attr:class="mt-6 h-px bg-slate-200"
                decorative=true
            />
        </div>
    }
}

/// A section within a component documentation page.
#[component]
pub fn DocSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="mb-12">
            <h2 class="mb-4 text-xl font-semibold text-slate-900">{title}</h2>
            {children()}
        </section>
    }
}

/// A part/sub-component description in the API reference.
#[component]
pub fn PartItem(name: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-baseline gap-3 py-2">
            <code class="shrink-0 rounded bg-slate-100 px-1.5 py-0.5 text-sm font-medium text-accent-700">{name}</code>
            <span class="text-sm text-slate-600">{description}</span>
        </div>
    }
}
