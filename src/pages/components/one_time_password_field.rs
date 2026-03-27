use leptos::prelude::*;
use pith_ui::one_time_password_field::*;

use crate::components::*;

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
                <DemoSection>
                    <div class="space-y-4">
                        <label class="block text-sm font-medium text-slate-700">"Enter verification code"</label>
                        <OneTimePasswordField
                            auto_submit=true
                            on_auto_submit=Callback::new(|code: String| {
                                web_sys::window().and_then(|w| w.alert_with_message(&format!("Code: {code}")).ok());
                            })
                        >
                            <div class="flex gap-2">
                                <OneTimePasswordFieldInput
                                    index=0
                                    attr:class="h-12 w-12 rounded-lg border border-slate-300 text-center text-lg font-semibold text-slate-900 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                />
                                <OneTimePasswordFieldInput
                                    index=1
                                    attr:class="h-12 w-12 rounded-lg border border-slate-300 text-center text-lg font-semibold text-slate-900 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                />
                                <OneTimePasswordFieldInput
                                    index=2
                                    attr:class="h-12 w-12 rounded-lg border border-slate-300 text-center text-lg font-semibold text-slate-900 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                />
                                <span class="flex items-center text-slate-400">"-"</span>
                                <OneTimePasswordFieldInput
                                    index=3
                                    attr:class="h-12 w-12 rounded-lg border border-slate-300 text-center text-lg font-semibold text-slate-900 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                />
                                <OneTimePasswordFieldInput
                                    index=4
                                    attr:class="h-12 w-12 rounded-lg border border-slate-300 text-center text-lg font-semibold text-slate-900 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                />
                                <OneTimePasswordFieldInput
                                    index=5
                                    attr:class="h-12 w-12 rounded-lg border border-slate-300 text-center text-lg font-semibold text-slate-900 focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                />
                            </div>
                            <OneTimePasswordFieldHiddenInput />
                        </OneTimePasswordField>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::one_time_password_field::*;

view! {
    <OneTimePasswordField
        on_complete=Callback::new(|code: String| { /* verify */ })
    >
        <div class="flex gap-2">
            <OneTimePasswordFieldInput index=0 />
            <OneTimePasswordFieldInput index=1 />
            <OneTimePasswordFieldInput index=2 />
            <OneTimePasswordFieldInput index=3 />
            <OneTimePasswordFieldInput index=4 />
            <OneTimePasswordFieldInput index=5 />
        </div>
        <OneTimePasswordFieldHiddenInput />
    </OneTimePasswordField>
}"# />
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
