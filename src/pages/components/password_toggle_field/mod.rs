use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::PasswordToggleFieldBasicSection;

#[component]
pub fn PasswordToggleFieldPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Password Toggle Field"
                description="A password input with a built-in visibility toggle button."
                features=&["Accessible", "Show/hide toggle", "Icon slots", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <PasswordToggleFieldBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="PasswordToggleField" description="Root component. Manages visibility state." />
                    <PartItem name="PasswordToggleFieldInput" description="The password input element." />
                    <PartItem name="PasswordToggleFieldSlot" description="Optional slot for custom content alongside the input." />
                    <PartItem name="PasswordToggleFieldIcon" description="Renders the appropriate icon based on visibility state." />
                    <PartItem name="PasswordToggleFieldToggle" description="Button that toggles password visibility." />
                </div>
            </DocSection>
        </div>
    }
}
