use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::RadioGroupBasicSection;

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
                <RadioGroupBasicSection />
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
