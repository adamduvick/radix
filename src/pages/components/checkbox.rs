use leptos::prelude::*;
use pith_ui::checkbox::*;
use pith_ui::label::Label;

use crate::components::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let (checked, set_checked) = signal(CheckedState::True);

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Checkbox"
                description="A control that allows the user to toggle between checked and unchecked states."
                features=&["Accessible", "Indeterminate state", "Form integration", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="space-y-4">
                        <div class="flex items-center gap-3">
                            <Checkbox
                                attr:id="terms"
                                attr:class="flex h-5 w-5 items-center justify-center rounded border border-slate-300 bg-white transition-colors data-[state=checked]:border-accent-600 data-[state=checked]:bg-accent-600"
                                checked=checked
                                on_checked_change=move |c| set_checked.set(c)
                            >
                                <CheckboxIndicator attr:class="text-white">
                                    {check_icon()}
                                </CheckboxIndicator>
                            </Checkbox>
                            <Label attr:r#for="terms" attr:class="text-sm font-medium text-slate-700 select-none cursor-pointer">
                                "Accept terms and conditions"
                            </Label>
                        </div>

                        <div class="flex items-center gap-3">
                            <Checkbox
                                attr:id="marketing"
                                default_checked=CheckedState::False
                                attr:class="flex h-5 w-5 items-center justify-center rounded border border-slate-300 bg-white transition-colors data-[state=checked]:border-accent-600 data-[state=checked]:bg-accent-600"
                            >
                                <CheckboxIndicator attr:class="text-white">
                                    {check_icon()}
                                </CheckboxIndicator>
                            </Checkbox>
                            <Label attr:r#for="marketing" attr:class="text-sm font-medium text-slate-700 select-none cursor-pointer">
                                "Send me marketing emails"
                            </Label>
                        </div>

                        <div class="flex items-center gap-3">
                            <Checkbox
                                attr:id="disabled"
                                disabled=true
                                attr:class="flex h-5 w-5 items-center justify-center rounded border border-slate-200 bg-slate-100 opacity-50 cursor-not-allowed"
                            >
                                <CheckboxIndicator attr:class="text-white">
                                    {check_icon()}
                                </CheckboxIndicator>
                            </Checkbox>
                            <Label attr:r#for="disabled" attr:class="text-sm text-slate-400 select-none cursor-not-allowed">
                                "Disabled option"
                            </Label>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::checkbox::*;

view! {
    <Checkbox
        default_checked=CheckedState::False
        attr:class="checkbox"
    >
        <CheckboxIndicator attr:class="indicator">
            // Your check icon
        </CheckboxIndicator>
    </Checkbox>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Checkbox" description="The root checkbox control. Renders a button with a hidden input." />
                    <PartItem name="CheckboxIndicator" description="Renders when the checkbox is checked or indeterminate." />
                </div>
            </DocSection>
        </div>
    }
}

fn check_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m3 7 3 3 5-5" />
        </svg>
    }
}
