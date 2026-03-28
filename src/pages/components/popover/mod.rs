use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::PopoverBasicSection;

#[component]
pub fn PopoverPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Popover"
                description="Displays floating content anchored to a trigger element."
                features=&["Accessible", "Focus management", "Collision-aware", "Dismissable", "Arrow support"]
            />

            <DocSection title="Demo">
                <PopoverBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Popover" description="Root component. Manages open/close state." />
                    <PartItem name="PopoverTrigger" description="Button that opens the popover." />
                    <PartItem name="PopoverAnchor" description="Optional custom anchor for positioning." />
                    <PartItem name="PopoverPortal" description="Portals content to the body." />
                    <PartItem name="PopoverContent" description="The floating popover panel." />
                    <PartItem name="PopoverClose" description="Button that closes the popover." />
                    <PartItem name="PopoverArrow" description="An arrow pointing to the trigger." />
                </div>
            </DocSection>
        </div>
    }
}
