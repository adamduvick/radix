use leptos::prelude::*;
use pith_ui::accordion::*;

use crate::components::*;

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
                <DemoSection>
                    <AccordionSingle
                        default_value="item-1".to_string()
                        collapsible=true
                        attr:class="w-full max-w-md divide-y divide-slate-200 rounded-lg border border-slate-200"
                    >
                        <AccordionItem value="item-1".to_string()>
                            <AccordionHeader>
                                <AccordionTrigger attr:class="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium text-slate-900 hover:bg-slate-50 transition-colors [&[data-state=open]>span]:rotate-180">
                                    "Is it accessible?"
                                    <span class="text-slate-500 transition-transform duration-200">{chevron_down()}</span>
                                </AccordionTrigger>
                            </AccordionHeader>
                            <AccordionContent attr:class="accordion-content overflow-hidden">
                                <div class="px-4 pb-3 text-sm text-slate-600">
                                    "Yes. It adheres to the WAI-ARIA Accordion pattern and supports full keyboard navigation."
                                </div>
                            </AccordionContent>
                        </AccordionItem>

                        <AccordionItem value="item-2".to_string()>
                            <AccordionHeader>
                                <AccordionTrigger attr:class="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium text-slate-900 hover:bg-slate-50 transition-colors [&[data-state=open]>span]:rotate-180">
                                    "Is it unstyled?"
                                    <span class="text-slate-500 transition-transform duration-200">{chevron_down()}</span>
                                </AccordionTrigger>
                            </AccordionHeader>
                            <AccordionContent attr:class="accordion-content overflow-hidden">
                                <div class="px-4 pb-3 text-sm text-slate-600">
                                    "Yes. It ships with zero styles, giving you full control over the look and feel."
                                </div>
                            </AccordionContent>
                        </AccordionItem>

                        <AccordionItem value="item-3".to_string()>
                            <AccordionHeader>
                                <AccordionTrigger attr:class="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium text-slate-900 hover:bg-slate-50 transition-colors [&[data-state=open]>span]:rotate-180">
                                    "Can it be animated?"
                                    <span class="text-slate-500 transition-transform duration-200">{chevron_down()}</span>
                                </AccordionTrigger>
                            </AccordionHeader>
                            <AccordionContent attr:class="accordion-content overflow-hidden">
                                <div class="px-4 pb-3 text-sm text-slate-600">
                                    "Yes. Use CSS animations with the "
                                    <code class="rounded bg-slate-100 px-1 text-xs">"data-state"</code>
                                    " attribute and "
                                    <code class="rounded bg-slate-100 px-1 text-xs">"--radix-accordion-content-height"</code>
                                    " custom property."
                                </div>
                            </AccordionContent>
                        </AccordionItem>
                    </AccordionSingle>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::accordion::*;

view! {
    <AccordionSingle
        default_value="item-1".to_string()
        collapsible=true
    >
        <AccordionItem value="item-1".to_string()>
            <AccordionHeader>
                <AccordionTrigger>"Section 1"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent>
                "Content for section 1."
            </AccordionContent>
        </AccordionItem>
    </AccordionSingle>
}"# />
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

fn chevron_down() -> impl IntoView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m4 6 4 4 4-4" />
        </svg>
    }
}
