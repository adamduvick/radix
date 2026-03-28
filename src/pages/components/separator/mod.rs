use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::SeparatorBasicSection;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Separator"
                description="Visually or semantically separates content."
                features=&["Accessible", "Horizontal & vertical", "Decorative mode"]
            />

            <DocSection title="Demo">
                <SeparatorBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Separator" description="A horizontal or vertical line. When decorative=true, it is hidden from screen readers." />
                </div>
            </DocSection>
        </div>
    }
}
