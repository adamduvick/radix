use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::ProgressBasicSection;

#[component]
pub fn ProgressPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Progress"
                description="Displays an indicator showing the completion progress of a task."
                features=&["Accessible", "Determinate & indeterminate", "Custom max value"]
            />

            <DocSection title="Demo">
                <ProgressBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Progress" description="The root progress container. Sets aria attributes." />
                    <PartItem name="ProgressIndicator" description="The visual indicator showing completion." />
                </div>
            </DocSection>
        </div>
    }
}
