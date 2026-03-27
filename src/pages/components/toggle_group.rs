use leptos::prelude::*;
use pith_ui::toggle_group::*;

use crate::components::*;

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Toggle Group"
                description="A set of two-state buttons that can be toggled on or off."
                features=&["Accessible", "Single or multiple selection", "Roving focus", "Keyboard navigation"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="space-y-6">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Text alignment (single)"</label>
                            <ToggleGroupSingle
                                default_value="center".to_string()
                                attr:class="inline-flex rounded-lg border border-slate-200 bg-slate-50 p-0.5"
                            >
                                <ToggleGroupItem
                                    value="left".to_string()
                                    attr:class="rounded-md px-3 py-1.5 text-sm font-medium text-slate-600 transition-colors data-[state=on]:bg-white data-[state=on]:text-slate-900 data-[state=on]:shadow-sm"
                                >
                                    "Left"
                                </ToggleGroupItem>
                                <ToggleGroupItem
                                    value="center".to_string()
                                    attr:class="rounded-md px-3 py-1.5 text-sm font-medium text-slate-600 transition-colors data-[state=on]:bg-white data-[state=on]:text-slate-900 data-[state=on]:shadow-sm"
                                >
                                    "Center"
                                </ToggleGroupItem>
                                <ToggleGroupItem
                                    value="right".to_string()
                                    attr:class="rounded-md px-3 py-1.5 text-sm font-medium text-slate-600 transition-colors data-[state=on]:bg-white data-[state=on]:text-slate-900 data-[state=on]:shadow-sm"
                                >
                                    "Right"
                                </ToggleGroupItem>
                            </ToggleGroupSingle>
                        </div>

                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Formatting (multiple)"</label>
                            <ToggleGroupMultiple
                                attr:class="inline-flex gap-0.5 rounded-lg border border-slate-200 bg-slate-50 p-0.5"
                            >
                                <ToggleGroupItem
                                    value="bold".to_string()
                                    attr:class="rounded-md px-3 py-1.5 text-sm font-bold text-slate-600 transition-colors data-[state=on]:bg-white data-[state=on]:text-accent-700 data-[state=on]:shadow-sm"
                                >
                                    "B"
                                </ToggleGroupItem>
                                <ToggleGroupItem
                                    value="italic".to_string()
                                    attr:class="rounded-md px-3 py-1.5 text-sm italic text-slate-600 transition-colors data-[state=on]:bg-white data-[state=on]:text-accent-700 data-[state=on]:shadow-sm"
                                >
                                    "I"
                                </ToggleGroupItem>
                                <ToggleGroupItem
                                    value="underline".to_string()
                                    attr:class="rounded-md px-3 py-1.5 text-sm underline text-slate-600 transition-colors data-[state=on]:bg-white data-[state=on]:text-accent-700 data-[state=on]:shadow-sm"
                                >
                                    "U"
                                </ToggleGroupItem>
                            </ToggleGroupMultiple>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::toggle_group::*;

// Single selection
view! {
    <ToggleGroupSingle default_value="center".to_string()>
        <ToggleGroupItem value="left".to_string()>"Left"</ToggleGroupItem>
        <ToggleGroupItem value="center".to_string()>"Center"</ToggleGroupItem>
        <ToggleGroupItem value="right".to_string()>"Right"</ToggleGroupItem>
    </ToggleGroupSingle>
}

// Multiple selection
view! {
    <ToggleGroupMultiple>
        <ToggleGroupItem value="bold".to_string()>"B"</ToggleGroupItem>
        <ToggleGroupItem value="italic".to_string()>"I"</ToggleGroupItem>
    </ToggleGroupMultiple>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="ToggleGroupSingle" description="Root for single-select toggle group." />
                    <PartItem name="ToggleGroupMultiple" description="Root for multi-select toggle group." />
                    <PartItem name="ToggleGroupItem" description="An individual toggle button within the group." />
                </div>
            </DocSection>
        </div>
    }
}
