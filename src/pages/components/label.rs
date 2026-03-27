use leptos::prelude::*;
use pith_ui::label::Label;

use crate::components::*;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Label"
                description="An accessible label that is associated with a form control."
                features=&["Accessible", "Click-to-focus", "For attribute support"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="space-y-4 max-w-sm">
                        <div>
                            <Label attr:r#for="email" attr:class="block text-sm font-medium text-slate-700 mb-1">
                                "Email address"
                            </Label>
                            <input
                                id="email"
                                type="email"
                                placeholder="you@example.com"
                                class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                            />
                        </div>
                        <div>
                            <Label attr:class="flex items-center gap-2 text-sm text-slate-700 cursor-pointer select-none">
                                <input type="checkbox" class="rounded border-slate-300" />
                                "Wrapped label (click anywhere)"
                            </Label>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::label::Label;

view! {
    <Label attr:r#for="name" attr:class="label">
        "Name"
    </Label>
    <input id="name" type="text" />
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Label" description="An accessible label element. Clicking it focuses the associated control." />
                </div>
            </DocSection>
        </div>
    }
}
