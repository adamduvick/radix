use leptos::prelude::*;
use pith_ui::toast::*;

use crate::components::*;

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
                <DemoSection>
                    <ToastProvider>
                        <ToastDemo />
                        <ToastViewport attr:class="toast-viewport" />
                    </ToastProvider>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::toast::*;

view! {
    <ToastProvider>
        <button on:click=|_| { /* trigger toast */ }>
            "Show Toast"
        </button>
        <Toast>
            <ToastTitle>"Success"</ToastTitle>
            <ToastDescription>"Your changes have been saved."</ToastDescription>
            <ToastClose>"Dismiss"</ToastClose>
        </Toast>
        <ToastViewport attr:class="toast-viewport" />
    </ToastProvider>
}"# />
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

#[component]
fn ToastDemo() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <button
            class="rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium text-white hover:bg-accent-700 transition-colors"
            on:click=move |_| set_open.set(true)
        >
            "Show Toast"
        </button>
        <Toast
            open=open
            on_open_change=Callback::new(move |v: bool| set_open.set(v))
            attr:class="rounded-lg border border-slate-200 bg-white p-4 shadow-lg"
        >
            <ToastTitle attr:class="text-sm font-semibold text-slate-900">
                "Changes saved"
            </ToastTitle>
            <ToastDescription attr:class="mt-1 text-sm text-slate-600">
                "Your profile has been updated successfully."
            </ToastDescription>
            <ToastClose attr:class="absolute right-2 top-2 rounded-md p-1 text-slate-400 hover:text-slate-600">
                "\u{2715}"
            </ToastClose>
        </Toast>
    }
}
