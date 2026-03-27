use leptos::prelude::*;
use pith_ui::form::*;

use crate::components::*;

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
                <DemoSection>
                    <Form
                        attr:class="w-full max-w-sm space-y-4"
                        on_clear_server_errors=Callback::new(|_| {})
                        on:submit=|event: web_sys::SubmitEvent| event.prevent_default()
                    >
                        <FormField name="name">
                            <FormLabel attr:class="block text-sm font-medium text-slate-700">"Name"</FormLabel>
                            <FormControl
                                attr:r#type="text"
                                attr:required=""
                                attr:class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                            />
                            <FormMessage
                                r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)
                                attr:class="mt-1 text-xs text-red-600"
                            >
                                "Name is required"
                            </FormMessage>
                        </FormField>

                        <FormField name="email">
                            <FormLabel attr:class="block text-sm font-medium text-slate-700">"Email"</FormLabel>
                            <FormControl
                                attr:r#type="email"
                                attr:required=""
                                attr:class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                            />
                            <FormMessage
                                r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)
                                attr:class="mt-1 text-xs text-red-600"
                            >
                                "Email is required"
                            </FormMessage>
                            <FormMessage
                                r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)
                                attr:class="mt-1 text-xs text-red-600"
                            >
                                "Please enter a valid email"
                            </FormMessage>
                        </FormField>

                        <FormSubmit attr:class="rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium text-white hover:bg-accent-700 transition-colors">
                            "Submit"
                        </FormSubmit>
                    </Form>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::form::*;

view! {
    <Form on_clear_server_errors=Callback::new(|_| {})>
        <FormField name="email">
            <FormLabel>"Email"</FormLabel>
            <FormControl attr:r#type="email" attr:required="" />
            <FormMessage r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)>
                "Email is required"
            </FormMessage>
            <FormMessage r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)>
                "Invalid email"
            </FormMessage>
        </FormField>
        <FormSubmit>"Submit"</FormSubmit>
    </Form>
}"# />
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
