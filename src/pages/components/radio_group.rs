use leptos::prelude::*;
use pith_ui::radio_group::*;

use crate::components::*;

#[component]
pub fn RadioGroupPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Radio Group"
                description="A set of checkable buttons where only one can be checked at a time."
                features=&["Accessible", "Keyboard navigation", "Arrow key cycling", "Form integration"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <RadioGroup
                        default_value="comfortable".to_string()
                        attr:class="space-y-3 max-w-sm"
                    >
                        <RadioItem label="Default" value="default" />
                        <RadioItem label="Comfortable" value="comfortable" />
                        <RadioItem label="Compact" value="compact" />
                    </RadioGroup>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::radio_group::*;

view! {
    <RadioGroup default_value="a".to_string()>
        <RadioGroupItem value="a".to_string()>
            <RadioGroupIndicator />
        </RadioGroupItem>
        <RadioGroupItem value="b".to_string()>
            <RadioGroupIndicator />
        </RadioGroupItem>
    </RadioGroup>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="RadioGroup" description="Root component. Manages the selected value." />
                    <PartItem name="RadioGroupItem" description="An individual radio button." />
                    <PartItem name="RadioGroupIndicator" description="The visual indicator for the selected state." />
                </div>
            </DocSection>
        </div>
    }
}

#[component]
fn RadioItem(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <label class="flex items-center gap-3 cursor-pointer select-none">
            <RadioGroupItem
                value=value.to_string()
                attr:class="flex h-5 w-5 items-center justify-center rounded-full border border-slate-300 bg-white transition-colors data-[state=checked]:border-accent-600"
            >
                <RadioGroupIndicator attr:class="block h-2.5 w-2.5 rounded-full bg-accent-600" />
            </RadioGroupItem>
            <span class="text-sm text-slate-700">{label}</span>
        </label>
    }
}
