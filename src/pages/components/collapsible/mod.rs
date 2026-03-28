use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::CollapsibleBasicSection;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Collapsible"
                description="A component that expands and collapses a panel of content."
                features=&["Accessible", "Animated", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <CollapsibleBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Collapsible" description="Root component. Manages open/closed state." />
                    <PartItem name="CollapsibleTrigger" description="Button that toggles the content visibility." />
                    <PartItem name="CollapsibleContent" description="The content area that expands/collapses." />
                </div>
            </DocSection>
        </div>
    }
}
