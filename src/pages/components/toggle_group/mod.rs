use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::ToggleGroupBasicSection;

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Toggle Group"
                description="A set of two-state buttons that can be toggled on or off."
                features=&["Accessible", "Single or multiple selection", "Roving focus", "Keyboard navigation"]
            />

            <DocSection title="Demo">
                <ToggleGroupBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="ToggleGroupSingle" description="Root for single-select toggle group." />
                    <PartItem name="ToggleGroupMultiple" description="Root for multi-select toggle group." />
                    <PartItem name="ToggleGroupItem" description="An individual toggle button within the group." />
                </div>
            </DocSection>
        </div>
    }
}
