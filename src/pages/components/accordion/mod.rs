use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::AccordionBasicSection;

#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Accordion"
                description="A vertically stacked set of interactive headings that each reveal a section of content."
                features=&["Accessible", "Keyboard navigation", "Single or multiple", "Collapsible", "Animated"]
            />

            <DocSection title="Demo">
                <AccordionBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="AccordionSingle" description="Root container. Only one item can be open at a time." />
                    <PartItem name="AccordionMultiple" description="Root container. Multiple items can be open simultaneously." />
                    <PartItem name="AccordionItem" description="Contains the trigger and content for a single section." />
                    <PartItem name="AccordionHeader" description="Wraps the trigger, rendered as an h3 by default." />
                    <PartItem name="AccordionTrigger" description="The button that toggles the section open/closed." />
                    <PartItem name="AccordionContent" description="The collapsible content area." />
                </div>
            </DocSection>
        </div>
    }
}
