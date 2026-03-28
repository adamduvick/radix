use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::LabelBasicSection;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Label"
                description="An accessible label that is associated with a form control."
                features=&["Accessible", "Click-to-focus", "For attribute support"]
            />

            <DocSection title="Demo">
                <LabelBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Label" description="An accessible label element. Clicking it focuses the associated control." />
                </div>
            </DocSection>
        </div>
    }
}
