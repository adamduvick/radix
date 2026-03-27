use leptos::prelude::*;
use pith_ui::collapsible::*;

use crate::components::*;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Collapsible"
                description="A component that expands and collapses a panel of content."
                features=&["Accessible", "Animated", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <Collapsible
                        open=open
                        on_open_change=Callback::new(move |v: bool| set_open.set(v))
                        attr:class="w-full max-w-sm"
                    >
                        <div class="flex items-center justify-between">
                            <span class="text-sm font-semibold text-slate-900">"3 repositories starred"</span>
                            <CollapsibleTrigger attr:class="inline-flex h-8 w-8 items-center justify-center rounded-md text-slate-500 hover:bg-slate-100 transition-colors">
                                {move || if open.get() { "\u{2212}" } else { "+" }}
                            </CollapsibleTrigger>
                        </div>

                        <div class="mt-2 rounded-md border border-slate-200 px-4 py-2.5 text-sm text-slate-700">
                            "pith-ui/pith-ui"
                        </div>

                        <CollapsibleContent attr:class="collapsible-content overflow-hidden">
                            <div class="mt-2 space-y-2">
                                <div class="rounded-md border border-slate-200 px-4 py-2.5 text-sm text-slate-700">
                                    "leptos-rs/leptos"
                                </div>
                                <div class="rounded-md border border-slate-200 px-4 py-2.5 text-sm text-slate-700">
                                    "radix-ui/primitives"
                                </div>
                            </div>
                        </CollapsibleContent>
                    </Collapsible>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::collapsible::*;

view! {
    <Collapsible>
        <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
        <CollapsibleContent>
            "Collapsible content here."
        </CollapsibleContent>
    </Collapsible>
}"# />
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
