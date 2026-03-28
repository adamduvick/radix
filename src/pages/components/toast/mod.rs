use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::ToastBasicSection;

#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Toast"
                description="A succinct message that is displayed temporarily as a notification."
                features=&["Accessible", "Auto dismiss", "Swipe to dismiss", "Pause on hover", "Multiple toasts"]
            />

            <DocSection title="Demo">
                <ToastBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="ToastProvider" description="Wraps your app to manage toast state." />
                    <PartItem name="ToastViewport" description="Fixed-position container where toasts appear." />
                    <PartItem name="Toast" description="An individual toast notification." />
                    <PartItem name="ToastTitle" description="The title text of the toast." />
                    <PartItem name="ToastDescription" description="The description text of the toast." />
                    <PartItem name="ToastAction" description="An action button within the toast." />
                    <PartItem name="ToastClose" description="Button to dismiss the toast." />
                </div>
            </DocSection>
        </div>
    }
}
