use leptos::prelude::*;
use pith_ui::form::*;

use crate::components::{extract_demo, DemoTabs};

const LABEL: &str = "block text-sm font-medium text-slate-700";
const INPUT: &str = "mt-1 w-full rounded-md border border-slate-300 px-3 py-2 \
    text-sm focus:border-accent-500 focus:outline-none focus:ring-1 \
    focus:ring-accent-500";
const MESSAGE: &str = "mt-1 text-xs text-red-600";
const SUBMIT: &str = "rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium \
    text-white hover:bg-accent-700 transition-colors";

// #region demo
#[component]
pub fn FormBasic() -> impl IntoView {
    view! {
        <Form
            attr:class="w-full max-w-sm space-y-4"
            on_clear_server_errors=Callback::new(|_| {})
            on:submit=|event: web_sys::SubmitEvent| event.prevent_default()
        >
            <FormField name="name">
                <FormLabel attr:class=LABEL>"Name"</FormLabel>
                <FormControl
                    attr:r#type="text"
                    attr:required=""
                    attr:class=INPUT
                />
                <FormMessage
                    r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)
                    attr:class=MESSAGE
                >
                    "Name is required"
                </FormMessage>
            </FormField>

            <FormField name="email">
                <FormLabel attr:class=LABEL>"Email"</FormLabel>
                <FormControl
                    attr:r#type="email"
                    attr:required=""
                    attr:class=INPUT
                />
                <FormMessage
                    r#match=Match::BuiltIn(ValidityMatcher::ValueMissing)
                    attr:class=MESSAGE
                >
                    "Email is required"
                </FormMessage>
                <FormMessage
                    r#match=Match::BuiltIn(ValidityMatcher::TypeMismatch)
                    attr:class=MESSAGE
                >
                    "Please enter a valid email"
                </FormMessage>
            </FormField>

            <FormSubmit attr:class=SUBMIT>
                "Submit"
            </FormSubmit>
        </Form>
    }
}
// #endregion demo

#[component]
pub fn FormBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A form with name and email fields demonstrating built-in validation with required fields and type checking."
        </p>

        <DemoTabs source=extract_demo(include_str!("form_basic.rs"))>
            <FormBasic />
        </DemoTabs>
    }
}
