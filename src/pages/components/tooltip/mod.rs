use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::TooltipBasicSection;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Tooltip"
                description="A popup that displays information related to an element on hover or keyboard focus."
                features=&["Accessible", "Delayed open", "Keyboard support", "Collision-aware positioning"]
            />

            <DocSection title="Demo">
                <TooltipBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="TooltipProvider" description="Wraps your app to provide shared tooltip delay behavior." />
                    <PartItem name="Tooltip" description="Root component. Manages open/close state." />
                    <PartItem name="TooltipTrigger" description="The element that triggers the tooltip on hover/focus." />
                    <PartItem name="TooltipPortal" description="Portals the tooltip content to the document body." />
                    <PartItem name="TooltipContent" description="The floating tooltip content." />
                    <PartItem name="TooltipArrow" description="An optional arrow pointing to the trigger." />
                </div>
            </DocSection>
        </div>
    }
}
