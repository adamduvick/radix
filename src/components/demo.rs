use leptos::prelude::*;
use pith_ui::separator::Separator;

/// A container for interactive component demos.
#[component]
pub fn DemoSection(children: Children) -> impl IntoView {
    view! {
        <div class="rounded-lg border border-slate-200 bg-white p-6 shadow-sm">
            {children()}
        </div>
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
pub fn DocSection(
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="mb-12">
            <h2 class="mb-4 text-xl font-semibold text-slate-900">{title}</h2>
            {children()}
        </section>
    }
}

/// A part/sub-component description in the API reference.
#[component]
pub fn PartItem(
    name: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-baseline gap-3 py-2">
            <code class="shrink-0 rounded bg-slate-100 px-1.5 py-0.5 text-sm font-medium text-accent-700">{name}</code>
            <span class="text-sm text-slate-600">{description}</span>
        </div>
    }
}
