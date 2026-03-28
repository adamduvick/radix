use leptos::prelude::*;
use pith_ui::one_time_password_field::*;

use crate::components::{extract_demo, DemoTabs};

const SLOT: &str = "h-12 w-12 rounded-lg border border-slate-300 text-center text-lg \
    font-semibold text-slate-900 focus:border-accent-500 focus:outline-none \
    focus:ring-1 focus:ring-accent-500";

// #region demo
#[component]
pub fn OneTimePasswordFieldBasic() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <label class="block text-sm font-medium text-slate-700">"Enter verification code"</label>
            <OneTimePasswordField
                auto_submit=true
                on_auto_submit=Callback::new(|code: String| {
                    web_sys::window().and_then(|w| w.alert_with_message(&format!("Code: {code}")).ok());
                })
            >
                <div class="flex gap-2">
                    <OneTimePasswordFieldInput index=0 attr:class=SLOT />
                    <OneTimePasswordFieldInput index=1 attr:class=SLOT />
                    <OneTimePasswordFieldInput index=2 attr:class=SLOT />
                    <span class="flex items-center text-slate-400">"-"</span>
                    <OneTimePasswordFieldInput index=3 attr:class=SLOT />
                    <OneTimePasswordFieldInput index=4 attr:class=SLOT />
                    <OneTimePasswordFieldInput index=5 attr:class=SLOT />
                </div>
                <OneTimePasswordFieldHiddenInput />
            </OneTimePasswordField>
        </div>
    }
}
// #endregion demo

#[component]
pub fn OneTimePasswordFieldBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A six-digit verification code input with auto-advance and auto-submit on completion."
        </p>

        <DemoTabs source=extract_demo(include_str!("one_time_password_field_basic.rs"))>
            <OneTimePasswordFieldBasic />
        </DemoTabs>
    }
}
