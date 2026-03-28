use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::TimeFieldBasicSection;

#[component]
pub fn TimeFieldPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Time Field"
                description="A segmented time input with individual fields for hours, minutes, seconds, and period."
                features=&["Accessible", "Segmented spinbutton input", "12h & 24h formats", "Configurable granularity", "Keyboard control"]
            />

            <DocSection title="Demo">
                <TimeFieldBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="TimeField" description="Root component. Manages time value and segment focus." />
                    <PartItem name="TimeFieldInput" description="Renders the segmented time input with spinbutton behavior." />
                </div>
            </DocSection>
        </div>
    }
}
