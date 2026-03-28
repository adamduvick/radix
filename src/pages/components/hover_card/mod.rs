use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::HoverCardBasicSection;

#[component]
pub fn HoverCardPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Hover Card"
                description="Displays rich content when hovering over a trigger, like a user profile preview."
                features=&["Accessible", "Open/close delay", "Collision-aware", "Text selection support"]
            />

            <DocSection title="Demo">
                <HoverCardBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="HoverCard" description="Root component. Manages open/close state with delay." />
                    <PartItem name="HoverCardTrigger" description="The element that triggers the hover card." />
                    <PartItem name="HoverCardPortal" description="Portals content to the body." />
                    <PartItem name="HoverCardContent" description="The floating content panel." />
                    <PartItem name="HoverCardArrow" description="An arrow pointing to the trigger." />
                </div>
            </DocSection>
        </div>
    }
}
