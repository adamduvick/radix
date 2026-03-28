use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::ToggleBasicSection;

#[component]
pub fn TogglePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Toggle"
                description="A two-state button that can be switched on or off."
                features=&["Accessible", "Keyboard toggle", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <ToggleBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Toggle" description="A button that can be toggled on or off. Exposes data-state='on'|'off'." />
                </div>
            </DocSection>
        </div>
    }
}
