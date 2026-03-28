use leptos::prelude::*;

use crate::components::*;
use super::demos::CheckboxBasicSection;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Checkbox"
                description="A control that allows the user to toggle between checked and unchecked states."
                features=&["Accessible", "Indeterminate state", "Form integration", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <CheckboxBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Checkbox" description="The root checkbox control. Renders a button with a hidden input." />
                    <PartItem name="CheckboxIndicator" description="Renders when the checkbox is checked or indeterminate." />
                </div>
            </DocSection>
        </div>
    }
}
