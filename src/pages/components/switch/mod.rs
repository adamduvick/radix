use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::SwitchBasicSection;

#[component]
pub fn SwitchPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Switch"
                description="A control that allows the user to toggle between on and off states."
                features=&["Accessible", "Keyboard toggle", "Form integration", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <SwitchBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Switch" description="The root switch control. Renders a button with a hidden input." />
                    <PartItem name="SwitchThumb" description="The thumb that slides between states." />
                </div>
            </DocSection>
        </div>
    }
}
