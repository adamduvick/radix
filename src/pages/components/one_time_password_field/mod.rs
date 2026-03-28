use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::OneTimePasswordFieldBasicSection;

#[component]
pub fn OneTimePasswordFieldPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="One-Time Password Field"
                description="A segmented input for entering one-time codes like 2FA verification or SMS codes."
                features=&["Accessible", "Auto-advance between segments", "Paste support", "Configurable length", "Input validation"]
            />

            <DocSection title="Demo">
                <OneTimePasswordFieldBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="OneTimePasswordField" description="Root component. Manages the code value and segment focus." />
                    <PartItem name="OneTimePasswordFieldInput" description="An individual character input segment." />
                    <PartItem name="OneTimePasswordFieldHiddenInput" description="Hidden input for form submission." />
                </div>
            </DocSection>
        </div>
    }
}
