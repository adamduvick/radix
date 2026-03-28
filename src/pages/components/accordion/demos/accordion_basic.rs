use leptos::prelude::*;
use pith_ui::accordion::*;

use crate::components::{extract_demo, DemoTabs};

const ROOT: &str = "w-full max-w-md divide-y divide-slate-200 rounded-lg border border-slate-200";
const TRIGGER: &str = "flex w-full items-center justify-between px-4 py-3 text-left text-sm \
    font-medium text-slate-900 hover:bg-slate-50 transition-colors \
    [&[data-state=open]>span]:rotate-180";
const CHEVRON: &str = "text-slate-500 transition-transform duration-200";
const CONTENT: &str = "accordion-content overflow-hidden";
const CONTENT_INNER: &str = "px-4 pb-3 text-sm text-slate-600";

fn chevron_down() -> impl IntoView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m4 6 4 4 4-4" />
        </svg>
    }
}

// #region demo
#[component]
pub fn AccordionBasic() -> impl IntoView {
    view! {
        <AccordionSingle
            default_value="item-1".to_string()
            collapsible=true
            attr:class=ROOT
        >
            <AccordionItem value="item-1".to_string()>
                <AccordionHeader>
                    <AccordionTrigger attr:class=TRIGGER>
                        "Is it accessible?"
                        <span class=CHEVRON>{chevron_down()}</span>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=CONTENT>
                    <div class=CONTENT_INNER>
                        "Yes. It adheres to the WAI-ARIA Accordion pattern and supports full keyboard navigation."
                    </div>
                </AccordionContent>
            </AccordionItem>

            <AccordionItem value="item-2".to_string()>
                <AccordionHeader>
                    <AccordionTrigger attr:class=TRIGGER>
                        "Is it unstyled?"
                        <span class=CHEVRON>{chevron_down()}</span>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=CONTENT>
                    <div class=CONTENT_INNER>
                        "Yes. It ships with zero styles, giving you full control over the look and feel."
                    </div>
                </AccordionContent>
            </AccordionItem>

            <AccordionItem value="item-3".to_string()>
                <AccordionHeader>
                    <AccordionTrigger attr:class=TRIGGER>
                        "Can it be animated?"
                        <span class=CHEVRON>{chevron_down()}</span>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=CONTENT>
                    <div class=CONTENT_INNER>
                        "Yes. Use CSS animations with the "
                        <code class="rounded bg-slate-100 px-1 text-xs">"data-state"</code>
                        " attribute and "
                        <code class="rounded bg-slate-100 px-1 text-xs">"--radix-accordion-content-height"</code>
                        " custom property."
                    </div>
                </AccordionContent>
            </AccordionItem>
        </AccordionSingle>
    }
}
// #endregion demo

#[component]
pub fn AccordionBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A collapsible accordion with three items, allowing one open section at a time."
        </p>

        <DemoTabs source=extract_demo(include_str!("accordion_basic.rs"))>
            <AccordionBasic />
        </DemoTabs>
    }
}
