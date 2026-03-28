use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::FormBasicSection;

#[component]
pub fn FormPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Form"
                description="A form primitive with built-in validation, accessible error messages, and server error support."
                features=&["Accessible", "Client-side validation", "Server error support", "Custom matchers", "Async validation"]
            />

            <DocSection title="Demo">
                <FormBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Form" description="Root form element with validation management." />
                    <PartItem name="FormField" description="Wraps a form field and its validation messages." />
                    <PartItem name="FormLabel" description="Accessible label for the field." />
                    <PartItem name="FormControl" description="The actual input element." />
                    <PartItem name="FormMessage" description="Validation message shown when a match condition is met." />
                    <PartItem name="FormValidityState" description="Render prop exposing the field's validity state." />
                    <PartItem name="FormSubmit" description="Submit button that triggers validation." />
                </div>
            </DocSection>
        </div>
    }
}
